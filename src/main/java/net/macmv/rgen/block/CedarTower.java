package net.macmv.rgen.block;

import net.minecraft.block.Block;
import net.minecraft.block.BlockWall;
import net.minecraft.block.material.Material;


public class CedarTower extends BlockWall {
    public CedarTower() {
        super(new Block(Material.WOOD).setHardness(2.0F).setResistance(10.0F));
    }

}
