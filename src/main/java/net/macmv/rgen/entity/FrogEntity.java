package net.macmv.rgen.entity;

import net.minecraft.entity.EntityCreature;
import net.minecraft.entity.SharedMonsterAttributes;
import net.minecraft.entity.ai.*;
import net.minecraft.entity.player.EntityPlayer;
import net.minecraft.world.World;

public class FrogEntity extends EntityCreature {

  public FrogEntity(World world) {
    super(world);
    this.setSize(0.375F, 0.375F);

    // maybe this will help them move cuter?
    this.stepHeight = 1;
  }

  @Override
  protected void initEntityAI() {
    this.tasks.addTask(0, new EntityAISwimming(this));
    this.tasks.addTask(1, new EntityAIPanic(this, 1.38F));
    this.tasks.addTask(3, new EntityAIAvoidEntity<>(this, EntityPlayer.class, 2.0F, 0.8F, 1.4F));
    this.tasks.addTask(5, new EntityAIWanderAvoidWater(this, 1.0F));
    this.tasks.addTask(6, new EntityAIWanderAvoidWater(this, 1.25F));
    this.tasks.addTask(7, new EntityAIWatchClosest(this, EntityPlayer.class, 6F));
    this.tasks.addTask(8, new EntityAILookIdle(this));
  }

  @Override
  protected void applyEntityAttributes() {
    super.applyEntityAttributes();
    this.getEntityAttribute(SharedMonsterAttributes.MAX_HEALTH).setBaseValue(3.0D);
    this.getEntityAttribute(SharedMonsterAttributes.MOVEMENT_SPEED).setBaseValue(0.3D);
  }

  @Override
  public void fall(float distance, float multiplier) {
    super.fall(distance, multiplier * 0.25f);
  }

  @Override
  public float getEyeHeight() {
    return this.height * 0.75f;
  }

  @Override
  protected boolean canDespawn() {
    return false;
  }
}
