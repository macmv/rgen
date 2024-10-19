package net.macmv.rgen.rust;

import net.minecraft.block.Block;
import net.minecraft.block.state.IBlockState;
import net.minecraft.client.Minecraft;
import net.minecraft.entity.player.EntityPlayerMP;
import net.minecraft.network.play.server.SPacketPlayerAbilities;
import net.minecraft.server.integrated.IntegratedServer;
import net.minecraft.util.ResourceLocation;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.text.TextComponentString;
import net.minecraft.util.text.TextFormatting;
import net.minecraft.world.World;
import net.minecraft.world.WorldServer;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.chunk.storage.RegionFileCache;
import net.minecraft.world.storage.ISaveHandler;
import net.minecraft.world.storage.WorldInfo;
import net.minecraftforge.common.DimensionManager;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.GameData;

import java.io.File;

public class RustGenerator {
  private static native void init_generator(long seed);
  private static native void init();
  private static native int reload_generator();
  private static native void build_chunk(char[] data, int x, int z);
  private static native void build_biomes(byte[] data, int x, int z);
  private static native void build_biomes_region(byte[] data, int cellX, int cellZ, int width, int height);
  private static native String[] debug_info(int x, int y, int z);
  private static native String get_biome_name_at(int x, int y, int z);
  private static native byte get_biome_at(int x, int z);

  // Helpers for the rust code.

  // Block name to block ID. This does not include the 4 metadata bits.
  private static int block_name_to_id(String name) {
    Block block = Block.getBlockFromName(name);
    if (block == null) {
      return 0;
    }

    return GameData.getBlockStateIDMap().get(block.getDefaultState()) >> 4;
  }

  // Block name to block ID. This does not include the 4 metadata bits.
  private static String block_id_to_name(int id) {
    return Block.REGISTRY.getObjectById(id).getRegistryName().toString();
  }

  // Returns the maximum block ID (not state ID).
  private static int max_block_id() {
    // This is real dumb, but this is only called once on load.
    int max = 0;
    for (Block block : Block.REGISTRY) {
      int id = Block.getIdFromBlock(block);
      if (id > max) {
        max = id;
      }
    }
    return max;
  }

  // Default metadata of a block.
  private static int lookup_default_meta(int id) {
    Block block = Block.getBlockById(id);
    return GameData.getBlockStateIDMap().get(block.getDefaultState()) & 0x0f;
  }

  // The properties of a block.
  private static PropType[] lookup_block_prop_types(int id) {
    Block block = Block.getBlockById(id);
    return PropType.lookup(block);
  }

  // A mapping from metadatas to property maps.
  private static PropMap[] lookup_block_prop_values(int id) {
    Block block = Block.getBlockById(id);
    return PropMap.lookup(block);
  }

  private static int biome_name_to_id(String name) {
    Biome biome = Biome.REGISTRY.getObject(new ResourceLocation(name));
    if (biome == null) {
      return 0;
    }

    return Biome.getIdForBiome(biome);
  }

  private static void print_warnings(String name) {
    Minecraft.getMinecraft().player.sendMessage(new TextComponentString(name + "\n\n" + TextFormatting.YELLOW + "Reload successful."));
  }

  private static void print_errors(String name) {
    Minecraft.getMinecraft().player.sendMessage(new TextComponentString(name + "\n\n" + TextFormatting.RED + "Failed to reload."));
  }

  private static short get_block(int dim, int x, int y, int z) {
    BlockPos pos = new BlockPos(x, y, z);
    World world = Minecraft.getMinecraft().getIntegratedServer().getWorld(dim);
    IBlockState state = world.getBlockState(pos);
    int meta = state.getBlock().getMetaFromState(state);
    int id = Block.getIdFromBlock(state.getBlock());
    return (short) ((id << 4) | meta);
  }

  private static void set_block(int dim, int x, int y, int z, short block) {
    BlockPos pos = new BlockPos(x, y, z);
    World world = Minecraft.getMinecraft().getIntegratedServer().getWorld(dim);
    world.setBlockState(pos, Block.getBlockById(block >> 4).getStateFromMeta(block & 15));
  }

  private static boolean active = false;

  public static void init(long seed) {
    if (!active) {
      System.loadLibrary("rgen_jni");
      init();
    }
    active = true;
    init_generator(seed);
  }

  @SideOnly(Side.CLIENT)
  public static void reload() {
    int res = reload_generator();
    if (res == 0) {
      int dimension = 0;

      Minecraft minecraft = Minecraft.getMinecraft();
      minecraft.player.sendMessage(new TextComponentString(TextFormatting.YELLOW + "Regenerating world..."));
      IntegratedServer server = minecraft.getIntegratedServer();

      // Load the nether, then load the overworld. This makes sure to re-create the world correctly.
      // System.out.println("[0]: Sending the player to the nether.");
      // server.getEntityFromUuid(minecraft.player.getUniqueID()).changeDimension(-1);

      System.out.println("[1]: Unloading all chunks.");

      // Step 1: Lookup the region files.
      WorldServer prevworld = server.getWorld(dimension);
      File save = prevworld.getChunkSaveLocation();
      String saveName = save.getName();
      File region = new File(save, "region");

      // Step 2: Kill the world.
      DimensionManager.setWorld(dimension, null, server);

      // Step 3: Remove the actual region files.
      System.out.println("Removing region files in " + region);
      RegionFileCache.clearRegionFileReferences();
      for (File f : region.listFiles()) {
        System.out.println("deleting " + f);
        f.delete();
      }

      // Step 4: Load the server world again.
      ISaveHandler saveHandler = server.getActiveAnvilConverter().getSaveLoader(saveName, true);
      WorldInfo info = saveHandler.loadWorldInfo();
      server.loadAllWorlds(server.getFolderName(), server.getWorldName(), info.getSeed(), info.getTerrainType(), info.getGeneratorOptions());
      WorldServer overworld = server.getWorld(dimension);

      // This updates all the other things (players, the player list, the player chunk map, etc)
      // to use the new world.
      EntityPlayerMP player = (EntityPlayerMP) prevworld.playerEntities.iterator().next();
      prevworld.removeEntityDangerously(player);
      player.isDead = false; // removeEntityDangerously sets `isDead` to true.
      player.setWorld(overworld);

      // `preparePlayer` removes the player from the old chunk map, among other things.
      server.getPlayerList().preparePlayer(player, prevworld);
      player.connection.setPlayerLocation(player.posX, player.posY, player.posZ, player.rotationYaw, player.rotationPitch);
      player.interactionManager.setWorld(overworld);
      player.connection.sendPacket(new SPacketPlayerAbilities(player.capabilities));
      server.getPlayerList().updateTimeAndWeatherForPlayer(player, overworld);
      server.getPlayerList().syncPlayerInventory(player);

      overworld.getPlayerChunkMap().addPlayer(player);
      overworld.spawnEntity(player);

      // Step 5: Reload the chunks on the client.
      minecraft.addScheduledTask(() -> {
        minecraft.renderGlobal.loadRenderers();
        minecraft.player.sendMessage(new TextComponentString(TextFormatting.YELLOW + "Regenerated world."));
      });
    }
  }

  public static boolean isActive() {
    return active;
  }

  public static String[] getDebugInfo(int x, int y, int z) {
    return debug_info(x, y, z);
  }
  public static String getBiomeAt(int x, int y, int z) {
    return get_biome_name_at(x, y, z);
  }

  public static void make_chunk(char[] data, int x, int z) {
    build_chunk(data, x, z);
  }
  public static void make_biomes(byte[] biomes, int x, int z) {
    build_biomes(biomes, x, z);
  }
  public static void make_biomes_region_4x4(byte[] biomes, int cellX, int cellZ, int width, int height) {
    build_biomes_region(biomes, cellX, cellZ, width, height);
  }
  public static byte biome_id_at(int x, int z) {
    return get_biome_at(x, z);
  }
}
