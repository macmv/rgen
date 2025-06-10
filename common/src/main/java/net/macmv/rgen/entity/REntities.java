package net.macmv.rgen.entity;

import net.macmv.rgen.RGen;
import net.macmv.rgen.entity.model.FrogModel;
import net.minecraft.client.model.ModelBase;
import net.minecraft.client.renderer.entity.RenderLiving;
import net.minecraft.entity.Entity;
import net.minecraft.entity.EntityLiving;
import net.minecraft.util.ResourceLocation;
import net.minecraft.world.World;
import net.minecraftforge.fml.client.registry.RenderingRegistry;
import net.minecraftforge.fml.common.registry.EntityEntry;
import net.minecraftforge.fml.common.registry.EntityEntryBuilder;
import net.minecraftforge.fml.relauncher.Side;
import net.minecraftforge.fml.relauncher.SideOnly;
import net.minecraftforge.registries.IForgeRegistry;

import javax.annotation.Nullable;
import java.util.ArrayList;
import java.util.function.Function;

public class REntities {
  private static final ArrayList<EntityEntry> entities = new ArrayList<>();
  private static int id = 0;

  public static final Entity FROG = register("frog", FrogEntity.class, FrogEntity::new);

  private static <T extends Entity> T register(String name, Class<T> entity, Function<World, T> factory) {
    entities.add(EntityEntryBuilder.<T>create().id(new ResourceLocation(RGen.MOD_ID, name), id).name(name).entity(entity).factory(factory).tracker(150, 1, false).build());
    id++;
    // TODO: Not sure if we really need this.
    return null;
  }

  public static void registerEntities(IForgeRegistry<EntityEntry> reg) {
    for (EntityEntry e : entities) {
      reg.register(e);
    }
  }

  @SideOnly(Side.CLIENT)
  public static void registerModels() {
    registerModel(FrogEntity.class, new FrogModel(), "frog");
  }

  private static <T extends EntityLiving> void registerModel(Class<T> entity, ModelBase model, String name) {
    RenderingRegistry.registerEntityRenderingHandler(entity, m -> new RenderLiving<T>(m, model, 1.0F) {
      @Nullable
      @Override
      protected ResourceLocation getEntityTexture(T entity) {
        return new ResourceLocation(RGen.MOD_ID, "textures/entities/" + name + ".png");
      }
    });
  }
}
