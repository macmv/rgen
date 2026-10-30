use std::{
  collections::{HashMap, HashSet},
  ops::Range,
};

use parser::{Parser, Token};

mod parser;

struct Config {
  renames:          HashMap<&'static str, &'static str>,
  /// Identifiers that get replaced anywhere they appear (unless they are used as a member, like
  /// `foo.NAME`).
  words:            HashMap<&'static str, &'static str>,
  /// Methods that only exist in 1.12 (metadata, creative tab listing, etc). Declarations of these
  /// are removed entirely.
  drop_methods:     HashSet<&'static str>,
  /// Annotations that don't exist on fabric. Arguments are removed too.
  drop_annotations: HashSet<&'static str>,
  /// Vanilla classes that have the same name as one of our classes. These are never imported, and
  /// are always written out in full.
  qualified:        HashSet<&'static str>,
  /// Constants that were renamed, like `MapColor.SAND`.
  members:          HashMap<(&'static str, &'static str), &'static str>,
  /// Block state properties that only exist in 1.12 (or are already added by the modern parent).
  drop_props:       HashSet<&'static str>,
  /// Calls, written as statements, that only exist in 1.12.
  drop_calls:       HashSet<&'static str>,
  /// Methods that have a `ShapeContext` argument now, and their new names.
  shape_methods:    HashMap<&'static str, &'static str>,
  /// Vanilla classes that make us implement `getCodec`.
  codec_parents:    HashSet<&'static str>,
}

fn main() {
  let filename = std::env::args().nth(1).expect("No filename provided");

  let source = std::fs::read_to_string(filename).expect("Failed to read file");
  let mut renames = HashMap::new();

  // util
  renames.insert("net.minecraft.util.math.AxisAlignedBB", "net.minecraft.util.shape.VoxelShape");
  renames.insert("net.minecraft.util.EnumFacing", "net.minecraft.util.math.Direction");
  renames.insert("net.minecraft.util.EnumHand", "net.minecraft.util.Hand");
  renames.insert("net.minecraft.util.ResourceLocation", "net.minecraft.util.Identifier");
  renames.insert("net.minecraft.util.IStringSerializable", "net.minecraft.util.StringIdentifiable");

  // blocks
  renames.insert("net.minecraft.block.BlockBush", "net.minecraft.block.PlantBlock");
  renames.insert("net.minecraft.block.BlockLog", "net.minecraft.block.PillarBlock");
  renames.insert("net.minecraft.init.Blocks", "net.minecraft.block.Blocks");
  renames.insert("net.minecraft.block.state.IBlockState", "net.minecraft.block.BlockState");
  renames.insert("net.minecraft.block.BlockPackedIce", "net.minecraft.block.IceBlock");
  renames.insert("net.minecraft.block.BlockRotatedPillar", "net.minecraft.block.PillarBlock");
  renames.insert("net.minecraft.block.BlockVine", "net.minecraft.block.VineBlock");
  renames.insert("net.minecraft.block.BlockDoor", "net.minecraft.block.DoorBlock");
  renames.insert("net.minecraft.block.BlockFence", "net.minecraft.block.FenceBlock");
  renames.insert("net.minecraft.block.BlockFenceGate", "net.minecraft.block.FenceGateBlock");
  renames.insert("net.minecraft.block.BlockTrapDoor", "net.minecraft.block.TrapdoorBlock");
  renames.insert("net.minecraft.block.BlockStairs", "net.minecraft.block.StairsBlock");
  renames.insert("net.minecraft.block.BlockLeaves", "net.macmv.rgen.compat.LeavesBase");
  renames.insert("net.minecraft.block.BlockFalling", "net.macmv.rgen.compat.FallingBase");
  renames.insert("net.minecraft.block.BlockHorizontal", "net.minecraft.block.HorizontalFacingBlock");
  renames.insert("net.minecraft.block.BlockSapling", "net.minecraft.block.SaplingBlock");
  renames.insert("net.minecraft.block.BlockDoublePlant", "net.minecraft.block.TallPlantBlock");
  renames.insert("net.minecraft.block.IGrowable", "net.minecraft.block.Fertilizable");
  renames.insert("net.minecraft.block.material.MapColor", "net.minecraft.block.MapColor");

  // These don't exist anymore, so there are polyfills in `compat`.
  renames.insert("net.minecraft.block.material.Material", "net.macmv.rgen.compat.Material");
  renames.insert("net.minecraft.block.SoundType", "net.macmv.rgen.compat.SoundType");

  renames.insert(
    "net.minecraft.block.properties.PropertyDirection",
    "net.minecraft.state.property.DirectionProperty",
  );
  renames.insert(
    "net.minecraft.block.properties.PropertyBool",
    "net.minecraft.state.property.BooleanProperty",
  );
  renames.insert(
    "net.minecraft.block.properties.PropertyEnum",
    "net.minecraft.state.property.EnumProperty",
  );
  renames.insert(
    "net.minecraft.block.properties.PropertyInteger",
    "net.minecraft.state.property.IntProperty",
  );
  renames
    .insert("net.minecraft.block.properties.IProperty", "net.minecraft.state.property.Property");

  // items
  renames.insert("net.minecraft.item.ItemBlock", "net.minecraft.item.BlockItem");
  renames.insert("net.minecraft.item.ItemDoor", "net.minecraft.item.TallBlockItem");

  renames.insert("net.minecraft.creativetab.CreativeTabs", "net.macmv.rgen.compat.CreativeTabs");

  // entities
  renames
    .insert("net.minecraft.entity.player.EntityPlayer", "net.minecraft.entity.player.PlayerEntity");
  renames.insert("net.minecraft.entity.EntityLivingBase", "net.minecraft.entity.LivingEntity");

  // world
  renames.insert("net.minecraft.world.IBlockAccess", "net.minecraft.world.BlockView");
  renames.insert("net.minecraft.util.EnumActionResult", "net.minecraft.util.ActionResult");
  renames.insert("net.minecraft.entity.player.EntityPlayerMP", "net.minecraft.server.network.ServerPlayerEntity");

  // client
  renames.insert("net.minecraft.client.Minecraft", "net.minecraft.client.MinecraftClient");

  let words = HashMap::from([
    ("EnumAxis", "net.minecraft.util.math.Direction.Axis"),
    ("LOG_AXIS", "AXIS"),
    ("isRemote", "isClient"),
  ]);

  let drop_methods = HashSet::from([
    "getMetaFromState",
    "getStateFromMeta",
    "damageDropped",
    "getSubBlocks",
    "getSubItems",
    "onSheared",
    "harvestBlock",
    "getSilkTouchDrop",
    "getBlockLayer",
    "getItemDropped",
    "getWoodType",
    "canSustainPlant",
    "getOffsetType",
    "getPickBlock",
    "getMapColor",
    "getSaplingDropChance",
    "canSilkHarvest",
    // TODO: These all need to be ported to their modern equivalents.
    "isOpaqueCube",
    "isFullCube",
    "getStateForPlacement",
    "onItemUse",
    "neighborChanged",
    "onBlockAdded",
    "getDrops",
    "isPassable",
    "getActualState",
    "setLightOpacity",
    "onBlockPlacedBy",
    "onEntityCollidedWithBlock",
    "getAmbientOcclusionLightValue",
    "isNormalCube",
    "tickRate",
    "canGrow",
    "canUseBonemeal",
    "getPlantType",
    "getPlant",
    "getOffset",
    "JungleFlower::getItem",
    "destroyBlockWithDrops",
    "canPlaceBlockAt",
    "canBlockStay",
    "onBlockHarvested",
    "getLightOpacity",
    "doesSideBlockRendering",
    "updateTick",
    "Sapling::grow",
    "Sapling::updateTick",
    "Sapling::generateTree",
    "LeavesBlock::updateTick",
    "LeavesBlock::destroy",
  ]);

  let drop_annotations = HashSet::from(["SideOnly", "Nonnull", "Nullable", "SubscribeEvent"]);

  let qualified = HashSet::from(["net.minecraft.block.PlantBlock"]);

  let members = HashMap::from([
    (("MapColor", "OBSIDIAN"), "BLACK"),
    (("MapColor", "SAND"), "PALE_YELLOW"),
    (("MapColor", "WOOD"), "OAK_TAN"),
    (("MapColor", "ORANGE_STAINED_HARDENED_CLAY"), "TERRACOTTA_ORANGE"),
  ]);

  // Properties with a single value can't exist anymore, so those are dropped for their class.
  let drop_props = HashSet::from(["LOG_AXIS", "CHECK_DECAY", "DECAYABLE", "FlowerBlock::TYPE"]);
  let drop_calls = HashSet::from([
    "setHarvestLevel",
    "setTickRandomly",
    "setCreativeTab",
    // These are in the block settings now.
    "setHardness",
    "setResistance",
    "setSoundType",
    "disableStats",
  ]);
  let shape_methods = HashMap::from([
    ("getBoundingBox", "getOutlineShape"),
    ("getCollisionBoundingBox", "getCollisionShape"),
  ]);
  let codec_parents = HashSet::from([
    "net.minecraft.block.Block",
    "net.minecraft.block.PlantBlock",
    "net.macmv.rgen.compat.FallingBase",
    "net.minecraft.block.HorizontalFacingBlock",
  ]);

  let config = Config {
    renames,
    words,
    drop_methods,
    drop_annotations,
    qualified,
    members,
    drop_props,
    drop_calls,
    shape_methods,
    codec_parents,
  };

  let output = config.process(source);

  print!("{}", output);
}

impl Config {
  fn keep_import(&self, path: &str) -> bool {
    if self.renames.contains_key(path) {
      return true;
    }

    if path.starts_with("net.minecraftforge") || path.starts_with("javax") {
      return false;
    }

    // Packages that are gone. Anything that still uses these needs to be ported by hand.
    const GONE: &[&str] = &[
      "net.minecraft.init.",
      "net.minecraft.tileentity.",
      "net.minecraft.stats.",
      "net.minecraft.client.renderer.",
      "net.minecraft.client.model.",
      "net.minecraft.network.",
      "net.minecraft.world.gen.feature.WorldGen",
      "net.minecraft.item.ItemMultiTexture",
      "net.minecraft.item.ItemSlab",
    ];
    if GONE.iter().any(|g| path.starts_with(g)) {
      return false;
    }

    // Known bad imports should get removed.
    match path {
      "net.minecraft.block.state.BlockStateContainer"
      | "net.minecraft.block.BlockPlanks"
      | "net.minecraft.block.BlockDirt"
      | "net.minecraft.creativetab.CreativeTabs"
      | "net.minecraft.util.BlockRenderLayer"
      | "net.minecraft.util.DamageSource"
      | "net.minecraft.util.NonNullList" => false,
      _ => true,
    }
  }

  /// If the parser is at the name of a method declaration, this returns the range of the whole
  /// declaration (including annotations and modifiers), and a parser positioned after it.
  fn method_range<'a>(
    &self,
    decl_start: usize,
    parser: &Parser<'a>,
  ) -> Option<(Range<usize>, Parser<'a>)> {
    let mut look = parser.clone();

    if look.next() != Some(Token::Punct) || look.slice() != "(" {
      return None;
    }

    let mut depth = 1;
    while depth > 0 {
      match look.next() {
        Some(Token::Punct) if look.slice() == "(" => depth += 1,
        Some(Token::Punct) if look.slice() == ")" => depth -= 1,
        None => return None,
        _ => {}
      }
    }

    // Skip `throws Foo, Bar`.
    let mut tok = look.next();
    if tok == Some(Token::Word) && look.slice() == "throws" {
      while !(look.slice() == "{" || look.slice() == ";") {
        tok = look.next();
        tok?;
      }
    }

    match (tok, look.slice()) {
      (Some(Token::Punct), "{") => {
        let mut depth = 1;
        while depth > 0 {
          match look.next() {
            Some(Token::Punct) if look.slice() == "{" => depth += 1,
            Some(Token::Punct) if look.slice() == "}" => depth -= 1,
            None => return None,
            _ => {}
          }
        }
      }
      // An abstract method.
      (Some(Token::Punct), ";") => {}
      _ => return None,
    }

    Some((decl_start..look.range().end, look))
  }

  fn drops_prop(&self, class_name: &str, prop: &str) -> bool {
    self.drop_props.contains(prop) || self.drop_props.contains(format!("{class_name}::{prop}").as_str())
  }

  fn process(&self, source: String) -> String {
    let mut imports = HashMap::<String, String>::new();

    if source.starts_with("// # only v12 #") {
      return String::new();
    }

    // Nested names that are easier to fix up before parsing.
    let source = source
      .replace("BlockLog.EnumAxis", "EnumAxis")
      .replace("BlockDoublePlant.EnumBlockHalf", "net.minecraft.block.enums.DoubleBlockHalf")
      .replace("EnumFacing.Plane", "net.minecraft.util.math.Direction.Type")
      .replace("BlockPlanks.EnumType", "net.minecraft.block.WoodType")
      .replace(" implements net.minecraftforge.common.IPlantable", "")
      // Saplings don't grow with bonemeal yet, see the dropped methods.
      .replace(" implements IGrowable", "");

    let mut output = Output::new(source.clone());
    let mut parser = Parser::new(&source);
    let mut package = String::new();
    let mut class_name = String::new();
    let mut ctor_param = String::new();

    // Eat the package statement
    if parser.next() == Some(Token::Word) && parser.slice() == "package" {
      while parser.slice() != ";" {
        parser.next();
        if parser.slice() != ";" {
          package.push_str(parser.slice());
        }
      }
    }

    while parser.next() == Some(Token::Word) && parser.slice() == "import" {
      let line_start = parser.range().start;

      let (path, span) = parse_path(&mut parser);
      if !self.keep_import(&path) {
        // +2 includes `;` and `\n`.
        output.replace(line_start..span.end + 2, "");
        continue;
      }

      let (_, last_part) = path.rsplit_once('.').unwrap();
      imports.insert(last_part.to_string(), path.clone());

      if let Some(new_path) = self.renames.get(path.as_str()) {
        if self.qualified.contains(new_path) {
          output.replace(line_start..span.end + 2, "");
        } else {
          output.replace(span, new_path);
        }
      }
    }

    // The start of the current declaration: right after the last `;`, `{`, or `}`.
    let mut decl_start = parser.range().start;

    while let Some(tok) = parser.next() {
      match tok {
        Token::FabricComment => {
          // The comment is formed like:
          // ```
          // /* #fabric: <...> */
          // ```

          const PREFIX: &str = "/* #fabric:";
          const SUFFIX: &str = "*/";

          let source = parser.slice();
          let comment_start = source.find(PREFIX).unwrap();
          let source_start = comment_start + PREFIX.len();
          let source_end = source.rfind(SUFFIX).unwrap();

          // Whitespace is included in the token, so only replace the comment itself.
          let start = parser.range().start + comment_start;
          let end = parser.range().start + source_end + SUFFIX.len();
          output.replace(start..end, source[source_start..source_end].trim());
        }
        Token::V12Comment => {
          // The comment is formed like:
          // ```
          // // #v12-start
          // <...>
          // // #v12-end
          // ```

          const PREFIX: &str = "// #v12-start";
          const SUFFIX: &str = "// #v12-end";

          let source = parser.slice();
          let source_start = source.find(PREFIX).unwrap() + PREFIX.len();
          let source_end = source.rfind(SUFFIX).unwrap();

          let mut indent = 0;
          let mut i = parser.range().start + source_start - PREFIX.len();
          loop {
            let Some(c) = parser.src.get(i - 1..i) else {
              break;
            };

            if c == " " {
              indent += 1;
              i -= 1;
            } else {
              break;
            }
          }

          let indent_str = " ".repeat(indent);

          let mut i = source_start;
          let eat_line = |i: &mut usize| {
            loop {
              let range = parser.range().start + *i..parser.range().start + *i + 1;
              match parser.src.get(range) {
                None => panic!("unexpected end of file"),
                Some("\n") => break,
                Some(_) => *i += 1,
              }
            }
            *i += 1;
          };
          eat_line(&mut i);
          while i < source_end {
            let start = i;
            eat_line(&mut i);

            if i > source_end {
              break;
            }

            let mut line = &parser.src[parser.range().start + start..parser.range().start + i];
            line = line.strip_prefix(&indent_str).unwrap_or(line);
            output.replace(
              parser.range().start + start..parser.range().start + i,
              &format!("{indent_str}// {line}"),
            );
          }
        }

        Token::Word
          if parser.slice() == "BlockStateContainer" && package == "net.macmv.rgen.block" =>
        {
          macro_rules! t {
            ($tok:expr) => {
              if parser.next() != Some($tok) {
                panic!("expected {:?}, found {:?}", $tok, parser.slice());
              }
            };
            ($tok:expr, $slice:expr) => {
              if parser.next() != Some($tok) && parser.slice() != $slice {
                panic!("expected {:?}, found {:?}", $slice, parser.slice());
              }
            };
          }

          let start = parser.range().start;

          t!(Token::Word, "createBlockState");
          t!(Token::Punct, "(");
          t!(Token::Punct, ")");
          t!(Token::Punct, "{");
          t!(Token::Word, "return");
          t!(Token::Word, "new");
          t!(Token::Word, "BlockStateContainer");
          t!(Token::Punct, "(");
          t!(Token::Word, "this");
          t!(Token::Punct, ",");

          let mut props = vec![];
          loop {
            if parser.next() != Some(Token::Word) {
              panic!("expected word, found {:?}", parser.slice());
            }
            let mut prop = String::new();
            prop.push_str(parser.slice());

            match parser.next() {
              Some(Token::Punct) if parser.slice() == "," => {
                props.push(prop);
              }
              Some(Token::Punct) if parser.slice() == ")" => {
                props.push(prop);
                break;
              }
              Some(Token::Punct) if parser.slice() == "." => {
                prop.push_str(parser.slice());
                if parser.next() != Some(Token::Word) {
                  panic!("expected word, found {:?}", parser.slice());
                }
                prop.push_str(parser.slice());

                props.push(prop);
                match parser.next() {
                  Some(Token::Punct) if parser.slice() == "," => {}
                  Some(Token::Punct) if parser.slice() == ")" => break,
                  _ => panic!("expected ',' or ')', found {:?}", parser.slice()),
                }
              }

              _ => panic!("expected ',', '.', or ')', found {:?}", parser.slice()),
            }
          }

          t!(Token::Punct, ";");
          t!(Token::Punct, "}");

          props.retain(|p| !self.drops_prop(&class_name, p));
          for p in &mut props {
            *p = p.replace("BlockHorizontal.", "net.minecraft.block.HorizontalFacingBlock.");
          }

          let end = parser.range().end;

          output.replace(
            start..end,
            &format!(
              "void appendProperties(net.minecraft.state.StateManager.Builder<net.minecraft.block.Block, net.minecraft.block.BlockState> builder) {{\n    super.appendProperties(builder);\n{}  }}",
              if props.is_empty() {
                String::new()
              } else {
                format!("    builder.add({});\n", props.join(", "))
              }
            ),
          );
        }
        Token::Word if parser.slice() == "super" && package == "net.macmv.rgen.block" => {
          if parser.next() == Some(Token::Punct) && parser.slice() == "(" {
            let mut look = parser.clone();
            if look.next() == Some(Token::Punct) && look.slice() == ")" && !ctor_param.is_empty() {
              output.replace(look.range(), &format!("{ctor_param}.vanilla())"));
              parser = look;
            } else if parser.next() == Some(Token::Word) {
              if parser.next() == Some(Token::Punct) && parser.slice() == "." {
                let dot_range = parser.range();
                if parser.next() == Some(Token::Word) && parser.slice() == "material" {
                  output.replace(dot_range, ".vanilla()");
                  output.replace(parser.range(), "");
                }
              }
            }
          }
        }
        Token::Word if parser.slice() == "blockState" && package == "net.macmv.rgen.block" => {
          let start = parser.range().start;
          if parser.next() == Some(Token::Punct) && parser.slice() == "." {
            if parser.next() == Some(Token::Word) && parser.slice() == "getBaseState" {
              output.replace(start..parser.range().end, "getDefaultState");
            }
          }
        }
        Token::Word if parser.slice() == "getName" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "asString");
        }
        // Property#create -> Property#of
        Token::Word if parser.slice() == "create" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "of");
        }
        // getValue(Property) -> get(Property)
        Token::Word if parser.slice() == "getValue" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "get");
        }
        Token::Word if parser.slice() == "NULL_AABB" && package == "net.macmv.rgen.block" => {
          output.replace(parser.range(), "net.minecraft.util.shape.VoxelShapes.empty()");
        }

        // `@SideOnly(Side.CLIENT)` and friends.
        Token::Punct if parser.slice() == "@" => {
          let start = parser.range().start;
          let mut look = parser.clone();
          if look.next() == Some(Token::Word) && self.drop_annotations.contains(look.slice()) {
            let mut end = look.range().end;
            let mut after = look.clone();
            if after.next() == Some(Token::Punct) && after.slice() == "(" {
              let mut depth = 1;
              while depth > 0 {
                match after.next() {
                  Some(Token::Punct) if after.slice() == "(" => depth += 1,
                  Some(Token::Punct) if after.slice() == ")" => depth -= 1,
                  None => break,
                  _ => {}
                }
              }
              end = after.range().end;
              look = after;
            }
            output.replace(start..end, "");
            parser = look;
          }
        }

        Token::Word
          if (self.drop_methods.contains(parser.slice())
            || self.drop_methods.contains(format!("{class_name}::{}", parser.slice()).as_str()))
            && !source[..parser.range().start].trim_end().ends_with('.') =>
        {
          let name = parser.slice();
          if let Some((range, end_parser)) = self.method_range(decl_start, &parser) {
            // Leave a note, so these can be found and ported later.
            output.replace(range, &format!("\n  // v12-to-fabric: dropped `{name}`\n"));
            parser = end_parser;
            decl_start = parser.range().end;
          }
        }

        Token::Word if parser.slice() == "withProperty" && package == "net.macmv.rgen.block" => {
          // `.withProperty(DROPPED, ...)` is removed completely.
          let name_range = parser.range();
          let mut look = parser.clone();
          if look.next() == Some(Token::Punct)
            && look.slice() == "("
            && look.next() == Some(Token::Word)
            && self.drops_prop(&class_name, look.slice())
          {
            let mut depth = 1;
            let mut inner = parser.clone();
            inner.next();
            while depth > 0 {
              match inner.next() {
                Some(Token::Punct) if inner.slice() == "(" => depth += 1,
                Some(Token::Punct) if inner.slice() == ")" => depth -= 1,
                None => break,
                _ => {}
              }
            }
            let before = &source[..name_range.start];
            let dot = before.trim_end().len() - 1;
            output.replace(dot..inner.range().end, "");
            parser = inner;
          } else {
            output.replace(name_range, "with");
          }
        }

        Token::Word
          if self.shape_methods.contains_key(parser.slice())
            && package == "net.macmv.rgen.block" =>
        {
          output.replace(parser.range(), self.shape_methods[parser.slice()]);

          let is_call = source[..parser.range().start].trim_end().ends_with('.');

          let mut look = parser.clone();
          if look.next() == Some(Token::Punct) && look.slice() == "(" {
            let start = look.range().end;
            let mut depth = 1;
            while depth > 0 {
              match look.next() {
                Some(Token::Punct) if look.slice() == "(" => depth += 1,
                Some(Token::Punct) if look.slice() == ")" => depth -= 1,
                None => break,
                _ => {}
              }
            }
            let end = look.range().start;
            // Some sources already have a manual `#fabric:` argument.
            if !source[start..end].contains("ShapeContext") {
              let arg = if is_call { ", context" } else { ", net.minecraft.block.ShapeContext context" };
              output.replace(end..end, arg);
            }
          }
        }

        // Statements like `this.setHarvestLevel("shovel", 0);`
        Token::Word if self.drop_calls.contains(parser.slice()) => {
          let before = source[..parser.range().start].trim_end();
          // Statements are removed entirely, and calls in a chain are removed from the chain.
          let (start, statement) = if before.ends_with("this.") {
            (before.len() - "this.".len(), true)
          } else if before.ends_with('.') {
            (before.len() - 1, false)
          } else if before.ends_with(';') || before.ends_with('{') || before.ends_with('}') {
            (parser.range().start, true)
          } else {
            (usize::MAX, true)
          };

          if start != usize::MAX {
            if let Some(args) = call_args(&parser) {
              let mut look = parser.clone();
              look.skip_to(args.close + 1);
              if !statement {
                output.replace(start..args.close + 1, "");
                parser = look;
              } else {
                let mut after = look.clone();
                if after.next() == Some(Token::Punct) && after.slice() == ";" {
                  output.replace(start..after.range().end, "");
                  parser = after;
                }
              }
            }
          }
        }

        Token::Word
          if parser.slice() == "class"
            && !source[..parser.range().start].trim_end().ends_with('.') =>
        {
          let mut look = parser.clone();
          if look.next() == Some(Token::Word) {
            class_name = look.slice().to_string();
          }

          // Find the parent class, and see if it makes us implement `getCodec`.
          let mut parent = None;
          let mut brace = None;
          while let Some(tok) = look.next() {
            if tok == Token::Word && look.slice() == "extends" && parent.is_none() {
              if look.next() == Some(Token::Word) {
                parent = Some(look.slice().to_string());
              }
            } else if tok == Token::Punct && look.slice() == "{" {
              brace = Some(look.range().end);
              break;
            }
          }

          if let (Some(parent), Some(brace)) = (parent, brace) {
            let resolved = imports.get(&parent).and_then(|p| self.renames.get(p.as_str()));
            if let Some(resolved) = resolved {
              if self.codec_parents.contains(resolved) {
                output.replace(
                  brace..brace,
                  &format!(
                    "\n  @Override\n  protected com.mojang.serialization.MapCodec<? extends {resolved}> getCodec() {{\n    return null;\n  }}\n"
                  ),
                );
              }
            }
          }
        }

        // `Foo(BlockSettings settings) {` -> `Foo(BlockSettings settings) { super(settings);`
        Token::Word if parser.slice() == "BlockSettings" && package == "net.macmv.rgen.block" => {
          let prev = source[..parser.range().start].trim_end();
          if prev.ends_with('(') && prev[..prev.len() - 1].trim_end().ends_with(&class_name) {
            let mut look = parser.clone();
            if look.next() == Some(Token::Word) {
              let param = look.slice().to_string();
              ctor_param = param.clone();
              if look.next() == Some(Token::Punct)
                && look.slice() == ")"
                && look.next() == Some(Token::Punct)
                && look.slice() == "{"
              {
                let brace = look.range().end;
                let mut first = look.clone();
                let mut has_super = false;
                loop {
                  match first.next() {
                    Some(Token::Word) if matches!(first.slice(), "super" | "this") => {
                      // `this.foo()` is not a constructor call.
                      has_super = first.next() == Some(Token::Punct) && first.slice() == "(";
                    }
                    // This comment is replaced with the call.
                    Some(Token::FabricComment) => has_super = true,
                    Some(Token::V12Comment) => continue,
                    _ => {}
                  }
                  break;
                }
                if !has_super {
                  output.replace(brace..brace, &format!(" super({param}.vanilla());"));
                }
              }
            }
          }
        }

        Token::Word if parser.slice() == "new" => {
          let start = parser.range().start;
          let mut look = parser.clone();
          if look.next() == Some(Token::Word) {
            let class = look.slice().to_string();
            let class_range = look.range();

            match class.as_str() {
              "AxisAlignedBB" => {
                output.replace(start..class_range.end, "net.minecraft.util.shape.VoxelShapes.cuboid");
                parser = look;
              }
              "ResourceLocation" => {
                output.replace(start..class_range.end, "net.minecraft.util.Identifier.of");
                parser = look;
              }
              // These all take item settings now.
              "ItemBlock" | "ItemDoor" | "Item" => {
                if let Some(args) = call_args(&look) {
                  let close = args.close;
                  if class == "Item" {
                    if source[args.open..close].trim().is_empty() {
                      output.replace(close..close, "net.macmv.rgen.compat.ItemSettings.create()");
                    }
                  } else {
                    let first_end = args.commas.first().copied().unwrap_or(close);
                    let block = source[args.open..first_end].trim();
                    output.replace(
                      close..close,
                      &format!(", net.macmv.rgen.compat.ItemSettings.forBlock({block})"),
                    );
                  }
                }
              }
              // Items with variants don't exist anymore, so this is just a block item.
              "ItemMultiTexture" => {
                if let Some(args) = call_args(&look) {
                  let first_end = args.commas.first().copied().unwrap_or(args.close);
                  let block = source[args.open..first_end].trim();
                  output.replace(
                    start..args.close + 1,
                    &format!("new net.minecraft.item.BlockItem({block}, net.macmv.rgen.compat.ItemSettings.forBlock({block}))"),
                  );
                  let mut after = look.clone();
                  after.skip_to(args.close + 1);
                  parser = after;
                }
              }
              _ => {}
            }
          }
        }

        Token::Word => {
          let word = parser.slice();
          let mut look = parser.clone();
          if look.next() == Some(Token::Punct) && look.slice() == "." && look.next() == Some(Token::Word) {
            if let Some(new_member) = self.members.get(&(word, look.slice())) {
              output.replace(look.range(), new_member);
            }
          }

          if let Some(new_word) = self.words.get(word) {
            if !source[..parser.range().start].trim_end().ends_with('.') {
              output.replace(parser.range(), new_word);
            }
          } else if let Some(resolved) = imports.get(word) {
            if let Some(new_path) = self.renames.get(resolved.as_str()) {
              let (_, new_imported) = new_path.rsplit_once('.').unwrap();
              let range = parser.range();
              if self.qualified.contains(new_path) {
                output.replace(range, new_path);
              } else {
                output.replace(range, new_imported);
              }
            }
          }
        }
        _ => {}
      }

      // Some of the arms above consume tokens, so check where the parser ended up.
      if matches!(parser.slice(), ";" | "{" | "}") && parser.range().len() == 1 {
        decl_start = parser.range().end;
      }
    }

    output.finish()
  }
}

struct CallArgs {
  /// Just after the `(`.
  open:   usize,
  /// The `)`.
  close:  usize,
  /// The top level `,` characters.
  commas: Vec<usize>,
}

/// If `parser` is on a word that is followed by a call, this returns where its arguments are.
fn call_args(parser: &Parser) -> Option<CallArgs> {
  let mut look = parser.clone();
  if look.next() != Some(Token::Punct) || look.slice() != "(" {
    return None;
  }

  let open = look.range().end;
  let mut depth = 1;
  let mut commas = vec![];
  loop {
    match look.next() {
      Some(Token::Punct) if look.slice() == "(" => depth += 1,
      Some(Token::Punct) if look.slice() == ")" => {
        depth -= 1;
        if depth == 0 {
          return Some(CallArgs { open, close: look.range().start, commas });
        }
      }
      Some(Token::Punct) if look.slice() == "," && depth == 1 => commas.push(look.range().start),
      None => return None,
      _ => {}
    }
  }
}

fn parse_path(parser: &mut Parser) -> (String, Range<usize>) {
  let mut path = String::new();
  let mut start = None;

  loop {
    if let Some(Token::Word) = parser.next() {
      if start.is_none() {
        start = Some(parser.range().start);

        if parser.slice() == "static" {
          continue;
        }
      }
      path.push_str(parser.slice());
      assert_eq!(parser.next(), Some(Token::Punct));
      match parser.slice() {
        "." => {
          path.push('.');
        }
        ";" => break (path, start.unwrap()..parser.range().start),
        _ => panic!("expected '.' or ';', found '{}'", parser.slice()),
      }
    } else if parser.slice() == "*" {
      assert_eq!(parser.next(), Some(Token::Punct));
      match parser.slice() {
        ";" => break (path, start.unwrap()..parser.range().start),
        _ => panic!("expected ';', found '{}'", parser.slice()),
      }
    } else {
      panic!();
    }
  }
}

// This collects edits to the original source. Edits use offsets into the original source, and are
// applied (in order) when the output is finished. An edit that starts inside an earlier edit is
// ignored, which lets an edit that covers a large range (like dropping a method) win over edits
// to the text it covers.
struct Output {
  source: String,
  edits:  Vec<(Range<usize>, String)>,
}

impl Output {
  pub fn new(source: String) -> Self { Self { source, edits: vec![] } }

  pub fn replace(&mut self, range: Range<usize>, str: &str) {
    self.edits.push((range, str.to_string()));
  }

  pub fn finish(mut self) -> String {
    // Stable, so edits that start at the same place stay in the order they were made.
    self.edits.sort_by(|a, b| a.0.start.cmp(&b.0.start).then(b.0.end.cmp(&a.0.end)));

    let mut out = String::new();
    let mut cursor = 0;
    for (range, text) in self.edits {
      if range.start < cursor {
        continue;
      }
      out.push_str(&self.source[cursor..range.start]);
      out.push_str(&text);
      cursor = range.end;
    }
    out.push_str(&self.source[cursor..]);
    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn output_works() {
    let mut output = Output::new("foo bar".into());

    output.replace(0..3, "bazzz");
    output.replace(4..7, "quuux");
    assert_eq!(output.finish(), "bazzz quuux");
  }

  #[test]
  fn big_edit_wins() {
    let mut output = Output::new("foo bar baz".into());

    output.replace(4..7, "quuux");
    output.replace(0..8, "");
    assert_eq!(output.finish(), "baz");
  }
}
