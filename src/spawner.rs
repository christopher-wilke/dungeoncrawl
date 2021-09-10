use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Health {
                current: 20,
                max: 20
            }
        )
    );
}

pub fn spawn_monster(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point
) {

    let (mut hp, mut name, mut glyph) = (0,"".to_string(), to_cp437('g'));

    if rng.roll_dice(1, 10) >= 5 {
        let goblin = goblin();
        hp = goblin.0;
        name = goblin.1;
        glyph = goblin.2;
    }
    else {
        let orc = orc();
        hp = orc.0;
        name = orc.1;
        glyph = orc.2;
    }

    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph
            },
            MovingRandomly{},
            Health{current: hp, max: hp},
            Name(name)
        )
    );
}

fn goblin() -> (i32, String, FontCharType) {
    (
        1,
        "Goblin".to_string(),
        to_cp437('g')
    )
}

fn orc() -> (i32, String, FontCharType) {
    (
        2,
        "Orc".to_string(),
        to_cp437('o')
    )
}