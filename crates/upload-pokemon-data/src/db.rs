use crate::pokemon_csv::PokemonCsv;
use inflector::Inflector;
use serde::{Serialize, Serializer};
use sqlx::{
    database::{HasArguments, HasValueRef},
    encode::IsNull,
    mysql::MySqlTypeInfo,
    Database, Encode, Decode, MySql, MySqlPool, Type,
};
use svix_ksuid::{Ksuid, KsuidLike};

#[derive(Debug, Clone)]
pub struct PokemonId(Ksuid);

#[derive(Debug, Clone)]
pub struct PokemonTableRow {
    pub id: PokemonId,
    pub name: String,
    pub slug: String,
    pub pokedex_id: u16,
    // abilities: Vec<String>,
    // typing: Vec<String>,
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
    pub height: u16,
    pub weight: u16,
    pub generation: u16,
    pub female_rate: Option<f32>,
    pub genderless: bool,
    pub legendary_or_mythical: bool,
    pub is_default: bool,
    pub forms_switchable: bool,
    pub base_experience: u16,
    pub capture_rate: u16,
    // egg_groups: Vec<String>,
    pub base_happiness: u16,
    // evolves_from: Option<String>,
    pub primary_color: String,
    pub number_pokemon_with_typing: f32,
    pub normal_attack_effectiveness: f32,
    pub fire_attack_effectiveness: f32,
    pub water_attack_effectiveness: f32,
    pub electric_attack_effectiveness: f32,
    pub grass_attack_effectiveness: f32,
    pub ice_attack_effectiveness: f32,
    pub fighting_attack_effectiveness: f32,
    pub poison_attack_effectiveness: f32,
    pub ground_attack_effectiveness: f32,
    pub fly_attack_effectiveness: f32,
    pub psychic_attack_effectiveness: f32,
    pub bug_attack_effectiveness: f32,
    pub rock_attack_effectiveness: f32,
    pub ghost_attack_effectiveness: f32,
    pub dragon_attack_effectiveness: f32,
    pub dark_attack_effectiveness: f32,
    pub steel_attack_effectiveness: f32,
    pub fairy_attack_effectiveness: f32,
}

impl From<PokemonCsv> for PokemonTableRow {
    fn from(
        PokemonCsv {
            name,
            pokedex_id,
            abilities: _,
            typing: _,
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
            height,
            weight,
            generation,
            female_rate,
            genderless,
            is_legendary_or_mythical,
            is_default,
            forms_switchable,
            base_experience,
            capture_rate,
            egg_groups: _,
            base_happiness,
            evolves_from: _,
            primary_color,
            number_pokemon_with_typing,
            normal_attack_effectiveness,
            fire_attack_effectiveness,
            water_attack_effectiveness,
            electric_attack_effectiveness,
            grass_attack_effectiveness,
            ice_attack_effectiveness,
            fighting_attack_effectiveness,
            poison_attack_effectiveness,
            ground_attack_effectiveness,
            fly_attack_effectiveness,
            psychic_attack_effectiveness,
            bug_attack_effectiveness,
            rock_attack_effectiveness,
            ghost_attack_effectiveness,
            dragon_attack_effectiveness,
            dark_attack_effectiveness,
            steel_attack_effectiveness,
            fairy_attack_effectiveness,
        }: PokemonCsv,
    ) -> Self {
        let slug = name.to_kebab_case();
        let id = PokemonId(Ksuid::new(None, None));
        PokemonTableRow {
            id,
            slug,
            name,
            pokedex_id,
            hp: hp.into(),
            attack: attack.into(),
            defense: defense.into(),
            special_attack: special_attack.into(),
            special_defense: special_defense.into(),
            speed: speed.into(),
            height,
            weight,
            generation: generation.into(),
            female_rate,
            genderless,
            legendary_or_mythical: is_legendary_or_mythical,
            is_default,
            forms_switchable,
            base_experience,
            capture_rate: capture_rate.into(),
            base_happiness: base_happiness.into(),
            primary_color,
            number_pokemon_with_typing,
            normal_attack_effectiveness,
            fire_attack_effectiveness,
            water_attack_effectiveness,
            electric_attack_effectiveness,
            grass_attack_effectiveness,
            ice_attack_effectiveness,
            fighting_attack_effectiveness,
            poison_attack_effectiveness,
            ground_attack_effectiveness,
            fly_attack_effectiveness,
            psychic_attack_effectiveness,
            bug_attack_effectiveness,
            rock_attack_effectiveness,
            ghost_attack_effectiveness,
            dragon_attack_effectiveness,
            dark_attack_effectiveness,
            steel_attack_effectiveness,
            fairy_attack_effectiveness,
        }
    }
}

pub async fn insert_pokemon(
    pool: MySqlPool,
    PokemonTableRow {
        id,
        name,
        slug,
        pokedex_id,
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        height,
        weight,
        generation,
        female_rate,
        genderless,
        legendary_or_mythical,
        is_default,
        forms_switchable,
        base_experience,
        capture_rate,
        base_happiness,
        primary_color,
        number_pokemon_with_typing,
        normal_attack_effectiveness,
        fire_attack_effectiveness,
        water_attack_effectiveness,
        electric_attack_effectiveness,
        grass_attack_effectiveness,
        ice_attack_effectiveness,
        fighting_attack_effectiveness,
        poison_attack_effectiveness,
        ground_attack_effectiveness,
        fly_attack_effectiveness,
        psychic_attack_effectiveness,
        bug_attack_effectiveness,
        rock_attack_effectiveness,
        ghost_attack_effectiveness,
        dragon_attack_effectiveness,
        dark_attack_effectiveness,
        steel_attack_effectiveness,
        fairy_attack_effectiveness,
    }: PokemonTableRow,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO pokemon (
            id,
            slug,
            name,
            pokedex_id,
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
            height,
            weight,
            generation,
            female_rate,
            genderless,
            legendary_or_mythical,
            is_default,
            forms_switchable,
            base_experience,
            capture_rate,
            base_happiness,
            primary_color,
            number_pokemon_with_typing,
            normal_attack_effectiveness,
            fire_attack_effectiveness,
            water_attack_effectiveness,
            electric_attack_effectiveness,
            grass_attack_effectiveness,
            ice_attack_effectiveness,
            fighting_attack_effectiveness,
            poison_attack_effectiveness,
            ground_attack_effectiveness,
            fly_attack_effectiveness,
            psychic_attack_effectiveness,
            bug_attack_effectiveness,
            rock_attack_effectiveness,
            ghost_attack_effectiveness,
            dragon_attack_effectiveness,
            dark_attack_effectiveness,
            steel_attack_effectiveness,
            fairy_attack_effectiveness
         )
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        "#,
        id,
        slug,
        name,
        pokedex_id,
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        height,
        weight,
        generation,
        female_rate,
        genderless,
        legendary_or_mythical,
        is_default,
        forms_switchable,
        base_experience,
        capture_rate,
        base_happiness,
        primary_color,
        number_pokemon_with_typing,
        normal_attack_effectiveness,
        fire_attack_effectiveness,
        water_attack_effectiveness,
        electric_attack_effectiveness,
        grass_attack_effectiveness,
        ice_attack_effectiveness,
        fighting_attack_effectiveness,
        poison_attack_effectiveness,
        ground_attack_effectiveness,
        fly_attack_effectiveness,
        psychic_attack_effectiveness,
        bug_attack_effectiveness,
        rock_attack_effectiveness,
        ghost_attack_effectiveness,
        dragon_attack_effectiveness,
        dark_attack_effectiveness,
        steel_attack_effectiveness,
        fairy_attack_effectiveness,
    )
    .execute(&pool)
    .await
}

impl PokemonId {
    pub fn new() -> Self {
        Self(Ksuid::new(None, None))
    }
}

impl<'q> Encode<'q, MySql> for PokemonId {
    fn encode_by_ref(&self, buf: &mut <MySql as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        let bytes: &[u8] = &self.0.to_base62().into_bytes();
        <&[u8] as Encode<MySql>>::encode(bytes, buf)
    }
}

impl<'r> Decode<'r, MySql> for PokemonId {
    fn decode(
        value: <MySql as HasValueRef<'r>>::ValueRef,
    ) -> Result<PokemonId, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&[u8] as Decode<MySql>>::decode(value)?;
        let base62_ksuid = std::str::from_utf8(&value)?;
        let ksuid = Ksuid::from_base62(&base62_ksuid)?;
        Ok(PokemonId(ksuid))
    }
}

impl Type<MySql> for PokemonId {
    fn type_info() -> <MySql as Database>::TypeInfo {
        <&[u8] as Type<MySql>>::type_info()
    }
    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <&[u8] as Type<MySql>>::compatible(ty)
    }
}

impl Serialize for PokemonId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let id = self.0.to_base62();
        serializer.serialize_str(&id)
    }
}
