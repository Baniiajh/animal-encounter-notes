#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk menyimpan hewan
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Animal {
    id: u64,
    name: String,
    count: u32,
}

// Storage key untuk data hewan
const ANIMAL_DATA: Symbol = symbol_short!("ANIMAL");

#[contract]
pub struct AnimalCounterContract;

#[contractimpl]
impl AnimalCounterContract {

    // Ambil semua data hewan
    pub fn get_animals(env: Env) -> Vec<Animal> {
        return env
            .storage()
            .instance()
            .get(&ANIMAL_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // Tambah hewan baru
    pub fn create_animal(env: Env, name: String, count: u32) -> String {

        let mut animals: Vec<Animal> = env
            .storage()
            .instance()
            .get(&ANIMAL_DATA)
            .unwrap_or(Vec::new(&env));

        let animal = Animal {
            id: env.prng().gen::<u64>(),
            name: name,
            count: count,
        };

        animals.push_back(animal);

        env.storage().instance().set(&ANIMAL_DATA, &animals);

        return String::from_str(&env, "Hewan berhasil ditambahkan");
    }

    // Hapus hewan berdasarkan id
    pub fn delete_animal(env: Env, id: u64) -> String {

        let mut animals: Vec<Animal> = env
            .storage()
            .instance()
            .get(&ANIMAL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..animals.len() {

            if animals.get(i).unwrap().id == id {

                animals.remove(i);

                env.storage().instance().set(&ANIMAL_DATA, &animals);

                return String::from_str(
                    &env,
                    "Data hewan berhasil dihapus"
                );
            }
        }

        return String::from_str(
            &env,
            "Hewan tidak ditemukan"
        );
    }

    // Update hewan
    pub fn update_animal(
        env: Env,
        id: u64,
        name: String,
        count: u32,
    ) -> String {

        let mut animals: Vec<Animal> = env
            .storage()
            .instance()
            .get(&ANIMAL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..animals.len() {

            let mut animal = animals.get(i).unwrap();

            if animal.id == id {

                animal.name = name;
                animal.count = count;

                animals.set(i, animal);

                env.storage()
                    .instance()
                    .set(&ANIMAL_DATA, &animals);

                return String::from_str(
                    &env,
                    "Hewan berhasil diupdate"
                );
            }
        }

        String::from_str(
            &env,
            "Hewan tidak ditemukan"
        )
    }
}

mod test;
