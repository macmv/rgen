package net.macmv.rgen.rust;

import net.minecraft.block.Block;
import net.minecraft.client.Minecraft;
import net.minecraft.client.entity.EntityPlayerSP;
import net.minecraft.client.gui.GuiDownloadTerrain;
import net.minecraft.client.multiplayer.WorldClient;
import net.minecraft.client.network.NetHandlerPlayClient;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.network.play.server.SPacketPlayerPosLook;
import net.minecraft.network.play.server.SPacketRespawn;
import net.minecraft.network.play.server.SPacketUnloadChunk;
import net.minecraft.scoreboard.Scoreboard;
import net.minecraft.server.integrated.IntegratedServer;
import net.minecraft.server.management.PlayerChunkMap;
import net.minecraft.server.management.PlayerChunkMapEntry;
import net.minecraft.util.ResourceLocation;
import net.minecraft.util.math.BlockPos;
import net.minecraft.util.text.TextComponentString;
import net.minecraft.util.text.TextFormatting;
import net.minecraft.world.GameType;
import net.minecraft.world.WorldServer;
import net.minecraft.world.WorldSettings;
import net.minecraft.world.biome.Biome;
import net.minecraft.world.chunk.Chunk;
import net.minecraft.world.gen.ChunkProviderServer;
import net.minecraftforge.registries.GameData;

import java.util.HashSet;
import java.util.Set;

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

  private static int block_name_to_id(String name) {
    Block block = Block.getBlockFromName(name);
    if (block == null) {
      return 0;
    }

    return GameData.getBlockStateIDMap().get(block.getDefaultState());
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

  private static boolean active = false;

  public static void init(long seed) {
    if (!active) {
      System.loadLibrary("rgen_jni");
      init();
    }
    active = true;
    init_generator(seed);
  }

  public static void reload() {
    int res = reload_generator();
    if (res == 0) {
      // TODO: Wipe out the world.

      int dimension = 0;

      Minecraft minecraft = Minecraft.getMinecraft();

      IntegratedServer server = minecraft.getIntegratedServer();
      WorldServer serverWorld = server.getWorld(dimension);
      ChunkProviderServer provider = serverWorld.getChunkProvider();

      // This unloads the world on the server.
      provider.queueUnloadAll();

      minecraft.player.onKillCommand();

      /*
      // This reloads the world on the client.
      WorldClient clientWorld = minecraft.world;
      NetHandlerPlayClient handler = minecraft.getConnection();

      // BlockPos pos = minecraft.player.getPosition();
      // minecraft.setDimensionAndSpawnPlayer(0);
      // minecraft.player.setPositionAndUpdate(pos.getX(), pos.getY(), pos.getZ());

      // Load the nether, then load the overworld. This makes sure to re-create the world correctly.
      handler.handleRespawn(new SPacketRespawn(1, serverWorld.getDifficulty(), serverWorld.getWorldInfo().getTerrainType(), serverWorld.getWorldInfo().getGameType()));
      handler.handleRespawn(new SPacketRespawn(dimension, serverWorld.getDifficulty(), serverWorld.getWorldInfo().getTerrainType(), serverWorld.getWorldInfo().getGameType()));
      handler.handlePlayerPosLook(new SPacketPlayerPosLook(minecraft.player.posX, minecraft.player.posY, minecraft.player.posZ, minecraft.player.cameraYaw, minecraft.player.cameraPitch, new HashSet<>(), 0));
       */

      // EntityPlayerSP player = Minecraft.getMinecraft().player;
      // IntegratedServer server = Minecraft.getMinecraft().getIntegratedServer();
      // WorldServer serverWorld = server.getWorld(0);
      // PlayerChunkMap chunkMap = serverWorld.getPlayerChunkMap();
      // ChunkProviderServer provider = serverWorld.getChunkProvider();
      //
      // for (Chunk chunk : provider.id2ChunkMap.values()) {
      //   // if (chunkMap.contains(chunk.x, chunk.z)) {
      //   //   PlayerChunkMapEntry entry = chunkMap.getEntry(chunk.x, chunk.z);
      //   //   chunkMap.removeEntry(entry);
      //   // }
      //   //
      //   // clientWorld.getChunkProvider().unloadChunk(chunk.x, chunk.z);
      //   // provider.queueUnload(chunk);
      //   // clientWorld.send.connection.sendPacket(new SPacketUnloadChunk(chunk.x, chunk.z));
      // }
      //
      // // Reload the world.
      // for (EntityPlayer player1 : serverWorld.playerEntities) {
      //   player1.changeDimension(1);
      //   // player1.changeDimension(0);
      // }
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
