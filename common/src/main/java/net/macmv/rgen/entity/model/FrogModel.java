package net.macmv.rgen.entity.model;

import net.minecraft.client.model.ModelBase;
import net.minecraft.client.model.ModelBox;
import net.minecraft.client.model.ModelRenderer;
import net.minecraft.entity.Entity;

public class FrogModel extends ModelBase {
  private final ModelRenderer main;
  private final ModelRenderer body_r1;
  private final ModelRenderer head;
  private final ModelRenderer head_r1;
  private final ModelRenderer frontLeft;
  private final ModelRenderer cube_r1;
  private final ModelRenderer LegFL;
  private final ModelRenderer cube_r2;
  private final ModelRenderer footFL;
  private final ModelRenderer cube_r3;
  private final ModelRenderer frontRight;
  private final ModelRenderer cube_r4;
  private final ModelRenderer LegFR;
  private final ModelRenderer cube_r5;
  private final ModelRenderer footFR;
  private final ModelRenderer cube_r6;
  private final ModelRenderer backLeft;
  private final ModelRenderer cube_r7;
  private final ModelRenderer legBL;
  private final ModelRenderer cube_r8;
  private final ModelRenderer footBL;
  private final ModelRenderer cube_r9;
  private final ModelRenderer backRight;
  private final ModelRenderer cube_r10;
  private final ModelRenderer legBR;
  private final ModelRenderer cube_r11;
  private final ModelRenderer footBR;
  private final ModelRenderer cube_r12;

  public FrogModel() {
    textureWidth = 32;
    textureHeight = 32;

    main = new ModelRenderer(this);
    main.setRotationPoint(-1.0F, 22.0F, 0.0F);


    body_r1 = new ModelRenderer(this);
    body_r1.setRotationPoint(0.0F, 2.0F, 3.0F);
    main.addChild(body_r1);
    setRotationAngle(body_r1, -0.3927F, 0.0F, 0.0F);
    body_r1.cubeList.add(new ModelBox(body_r1, 0, 15, -1.0F, -3.0F, -6.0F, 4, 3, 6, 0.0F, false));

    head = new ModelRenderer(this);
    head.setRotationPoint(1.0F, -1.5F, -1.75F);
    main.addChild(head);
    head.cubeList.add(new ModelBox(head, 22, 20, 0.75F, -2.5F, -1.5F, 2, 2, 2, -0.4F, false));
    head.cubeList.add(new ModelBox(head, 22, 20, -2.75F, -2.5F, -1.5F, 2, 2, 2, -0.4F, true));

    head_r1 = new ModelRenderer(this);
    head_r1.setRotationPoint(-1.0F, 0.5F, -0.25F);
    head.addChild(head_r1);
    setRotationAngle(head_r1, -0.829F, 0.0F, 0.0F);
    head_r1.cubeList.add(new ModelBox(head_r1, 17, 13, -1.0F, -2.0499F, -1.6599F, 4, 3, 2, -0.01F, false));

    frontLeft = new ModelRenderer(this);
    frontLeft.setRotationPoint(3.0F, -0.5F, -1.0F);
    main.addChild(frontLeft);


    cube_r1 = new ModelRenderer(this);
    cube_r1.setRotationPoint(-0.25F, 1.5F, 0.75F);
    frontLeft.addChild(cube_r1);
    setRotationAngle(cube_r1, 0.7854F, 0.0F, 0.0F);
    cube_r1.cubeList.add(new ModelBox(cube_r1, 12, 2, 0.0F, -2.0F, 0.0F, 1, 2, 1, 0.0F, true));

    LegFL = new ModelRenderer(this);
    LegFL.setRotationPoint(0.5F, 0.75F, 0.75F);
    frontLeft.addChild(LegFL);


    cube_r2 = new ModelRenderer(this);
    cube_r2.setRotationPoint(-0.5F, 0.25F, 0.25F);
    LegFL.addChild(cube_r2);
    setRotationAngle(cube_r2, -0.5672F, 0.0F, 0.0F);
    cube_r2.cubeList.add(new ModelBox(cube_r2, 12, 6, 0.0F, 0.0F, -1.0F, 1, 2, 1, 0.0F, true));

    footFL = new ModelRenderer(this);
    footFL.setRotationPoint(0.5F, 2.5F, -0.5F);
    frontLeft.addChild(footFL);


    cube_r3 = new ModelRenderer(this);
    cube_r3.setRotationPoint(0.0F, 0.0F, -1.0F);
    footFL.addChild(cube_r3);
    setRotationAngle(cube_r3, 0.0F, 0.8727F, 0.0F);
    cube_r3.cubeList.add(new ModelBox(cube_r3, 18, 3, -1.0F, -0.1F, -1.0F, 2, 0, 2, 0.0F, true));

    frontRight = new ModelRenderer(this);
    frontRight.setRotationPoint(-1.0F, -0.5F, -1.0F);
    main.addChild(frontRight);


    cube_r4 = new ModelRenderer(this);
    cube_r4.setRotationPoint(0.25F, 1.5F, 0.75F);
    frontRight.addChild(cube_r4);
    setRotationAngle(cube_r4, 0.7854F, 0.0F, 0.0F);
    cube_r4.cubeList.add(new ModelBox(cube_r4, 12, 2, -1.0F, -2.0F, 0.0F, 1, 2, 1, 0.0F, false));

    LegFR = new ModelRenderer(this);
    LegFR.setRotationPoint(-0.5F, 1.0F, 1.0F);
    frontRight.addChild(LegFR);


    cube_r5 = new ModelRenderer(this);
    cube_r5.setRotationPoint(0.5F, 0.0F, 0.0F);
    LegFR.addChild(cube_r5);
    setRotationAngle(cube_r5, -0.5672F, 0.0F, 0.0F);
    cube_r5.cubeList.add(new ModelBox(cube_r5, 12, 6, -1.0F, 0.0F, -1.0F, 1, 2, 1, 0.0F, false));

    footFR = new ModelRenderer(this);
    footFR.setRotationPoint(0.0F, 1.75F, -1.5F);
    LegFR.addChild(footFR);


    cube_r6 = new ModelRenderer(this);
    cube_r6.setRotationPoint(0.0F, -0.25F, -1.0F);
    footFR.addChild(cube_r6);
    setRotationAngle(cube_r6, 0.0F, -0.8727F, 0.0F);
    cube_r6.cubeList.add(new ModelBox(cube_r6, 18, 3, -1.0F, -0.1F, -1.0F, 2, 0, 2, 0.0F, false));

    backLeft = new ModelRenderer(this);
    backLeft.setRotationPoint(3.0F, 0.5F, 3.0F);
    main.addChild(backLeft);


    cube_r7 = new ModelRenderer(this);
    cube_r7.setRotationPoint(-0.25F, 1.0F, 0.5F);
    backLeft.addChild(cube_r7);
    setRotationAngle(cube_r7, -0.3927F, -0.3927F, 0.1766F);
    cube_r7.cubeList.add(new ModelBox(cube_r7, 0, 0, -1.0F, -2.0F, -2.0F, 2, 2, 3, 0.0F, true));

    legBL = new ModelRenderer(this);
    legBL.setRotationPoint(-0.25F, 2.0F, 0.5F);
    backLeft.addChild(legBL);


    cube_r8 = new ModelRenderer(this);
    cube_r8.setRotationPoint(0.0F, 0.0F, 0.0F);
    legBL.addChild(cube_r8);
    setRotationAngle(cube_r8, 0.0F, -0.3927F, 0.0F);
    cube_r8.cubeList.add(new ModelBox(cube_r8, 0, 7, -1.0F, -1.5F, -1.0F, 2, 1, 2, 0.0F, true));

    footBL = new ModelRenderer(this);
    footBL.setRotationPoint(0.0F, -0.5F, 0.0F);
    legBL.addChild(footBL);


    cube_r9 = new ModelRenderer(this);
    cube_r9.setRotationPoint(0.0F, 0.5F, 0.0F);
    footBL.addChild(cube_r9);
    setRotationAngle(cube_r9, 0.0F, -0.3927F, 0.0F);
    cube_r9.cubeList.add(new ModelBox(cube_r9, 21, 3, -1.0F, -0.6F, -3.0F, 2, 0, 4, 0.0F, true));

    backRight = new ModelRenderer(this);
    backRight.setRotationPoint(-1.0F, 0.5F, 3.0F);
    main.addChild(backRight);


    cube_r10 = new ModelRenderer(this);
    cube_r10.setRotationPoint(0.25F, 1.0F, 0.5F);
    backRight.addChild(cube_r10);
    setRotationAngle(cube_r10, -0.3927F, 0.3927F, -0.1766F);
    cube_r10.cubeList.add(new ModelBox(cube_r10, 0, 0, -1.0F, -2.0F, -2.0F, 2, 2, 3, 0.0F, false));

    legBR = new ModelRenderer(this);
    legBR.setRotationPoint(0.0F, 0.0F, -3.0F);
    backRight.addChild(legBR);


    cube_r11 = new ModelRenderer(this);
    cube_r11.setRotationPoint(0.25F, 2.0F, 3.5F);
    legBR.addChild(cube_r11);
    setRotationAngle(cube_r11, 0.0F, 0.3927F, 0.0F);
    cube_r11.cubeList.add(new ModelBox(cube_r11, 0, 7, -1.0F, -1.5F, -1.0F, 2, 1, 2, 0.0F, false));

    footBR = new ModelRenderer(this);
    footBR.setRotationPoint(0.25F, 1.5F, 3.5F);
    legBR.addChild(footBR);


    cube_r12 = new ModelRenderer(this);
    cube_r12.setRotationPoint(0.0F, 0.5F, 0.0F);
    footBR.addChild(cube_r12);
    setRotationAngle(cube_r12, 0.0F, 0.3927F, 0.0F);
    cube_r12.cubeList.add(new ModelBox(cube_r12, 21, 3, -1.0F, -0.6F, -3.0F, 2, 0, 4, 0.0F, false));
  }

  @Override
  public void render(Entity entity, float f, float f1, float f2, float f3, float f4, float f5) {
    main.render(f5);
  }

  public void setRotationAngle(ModelRenderer modelRenderer, float x, float y, float z) {
    modelRenderer.rotateAngleX = x;
    modelRenderer.rotateAngleY = y;
    modelRenderer.rotateAngleZ = z;
  }
}