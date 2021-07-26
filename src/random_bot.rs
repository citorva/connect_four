//! Implémentation d’une intelligence artificielle plaçant systématiquement aléatoirement un jeton

use super::connect_four::{Interface, Area, State};

/// Intelligence artificielle aléatoire
pub struct RandomBot {
    #[doc(hidden)]
    name : String
}

impl RandomBot {
    /// Initialise l’intelligence artificielle
    ///
    /// # Arguments
    ///
    ///  * `name` : Le nom donné à l’intelligence artificielle.
    ///
    /// # Retour
    ///
    /// Une instance de l’intelligence artificielle
    pub fn new(name : &str) -> Self {
        Self {
            name: String::from(name)
        }
    }
}

impl Interface for RandomBot {
    fn play(&mut self, area: &Area, _ : State) -> usize {
        let available = area.get_available_columns();
        const SZ : usize = std::mem::size_of::<usize>();

        let mut idx_a : [u8; SZ] = [0; SZ];

        getrandom::getrandom(&mut idx_a).unwrap();

        let idx : usize = unsafe { std::mem::transmute::<[u8; SZ], usize>(idx_a) } % available.len();

        return available[idx];
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
}