//! Gestionnaire des parties virtuelles de puissance 4
//!
//! # Fonctionnalités
//!
//! Fournit un ensemble d’outils afin de gérer une partie de puissance 4.
//!
//! Ce dernier fournit trois éléments conçu pour cela:
//!  * L’objet [`Engine`] permettant de gérer une partie de puissance 4 ainsi que les interactions entre les joueurs et le jeu.
//!  * L’objet [`Area`] correspondant à l’aire de jeu (ici le plateau virtuel de puissance 4)
//!  * Le trait [`Interface`] permettant d’implémenter les fonctions essentielles entre l’interface utilisateur et le jeu.
//!
//! # Exemple
//!
//! Voici un exemple faisant jouer deux intelligences artificielles de manière aléatoire:
//!
//! ```rust
//! extern crate getrandom;
//!
//! mod connect_four;
//!
//! use super::connect_four::{Interface,Area};
//!
//! pub struct RandomBot {
//!     name : String
//! }
//!
//! impl RandomBot {
//!     pub fn new(name : &str) -> Self {
//!         Self {
//!             name: String::from(name)
//!         }
//!     }
//! }
//!
//! impl Interface for RandomBot {
//!     fn play(&self, area: &Area) -> usize {
//!         let available = area.get_available_columns();
//!         const SZ : usize = std::mem::size_of::<usize>();
//!
//!         let mut idx_a : [u8; SZ] = [0; SZ];
//!
//!         getrandom::getrandom(&mut idx_a).unwrap();
//!
//!         let idx : usize = unsafe { std::mem::transmute::<[u8; SZ], usize>(idx_a) } % available.len();
//!
//!         return available[idx];
//!     }
//!
//!     fn name(&self) -> String {
//!         return self.name.clone();
//!     }
//! }
//!
//! fn main() {
//!     let p1 = RandomBot::new("Joueur 1");
//!     let p2 = RandomBot::new("Joueur 2");
//!
//!     let mut game = p4e::Engine::new(&p1, &p2);
//!
//!     if let Ok(v) = game.play() {
//!         println!("{}", game.get_disposition());
//!
//!         if let Some(p) = v {
//!             println!("{} a gagné", p);
//!         } else {
//!             println!("Match nul");
//!         }
//!     }
//! }
//! ```
//!
//! [`Engine`]: struct.Engine.html
//! [`Area`]: struct.Area.html
//! [`Interface`]: trait.Interface.html

use std::fmt::{Debug, Formatter, Display};
use std::cell::RefCell;

/// Nombre de lignes sur le plateau.
///
/// Nombre de jetons pouvant être verticalement alignés.
///
/// Défini directement la zone de jeu et vaut 6 par default.
pub const AREA_ROWS : usize = 6;
/// Nombre de colonnes sur le plateau.
///
/// Nombre de jetons pouvant être horizontalement alignés.
///
/// Défini directement la zone de jeu et vaut 7 par default.
pub const AREA_COLS : usize = 7;

/// Nombre de jetons à aligner afin d’enclencher la victoire.
///
/// Ce dernier correspond au nombre de jetons à aligner horizontalement, verticalement ou en
/// diagonale et permettant à l’un des deux joueurs de gagner.
pub const VICTORY_NUMBER : usize = 4;

/// Un type [`Result`] spécialisé aux opérations du moteur de jeu.
///
/// Ce type est utilisé dans tout le module [`connect_four`] pour toutes les opération pouvant
/// émettre une erreur.
///
/// Cet alias est défini dans le but de limiter le nombre de réécriture de [`connect_four::Error`].
///
/// [`Result`]: std::result::Result
/// [`connect_four`]: self
/// [`connect_four::Error`]: Error
pub type Result<T> = std::result::Result<T, Error>;

/// État d’une case dans la zone de jeu.
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum State {
    /// La case ne contient pas de jeton. Cette valeur est interne à l’objet Area
    NoToken,
    /// La case contient un jeton rouge
    RedToken,
    /// La case contient un jeton jaune
    YellowToken,
}

/// Liste des erreurs pouvant être émises par les objet du module
pub enum Error {
    /// L’identifiant de la colonne est invalide. Cette dernière doit être comprise entre 0 et `AREA_COLS-1`.
    InvalidColumn,
    /// L’état demandé ne correspond pas à celui d’un jeton.
    NotAToken,
    /// La colonne dont il est demandé une modification est déjà remplie.
    FilledColumn,
    /// L’identifiant du joueur est invalide (doit valoir 0 ou 1)
    InvalidPlayerId(usize),
}

/// Gère les parties de puissance 4.
///
/// Ce dernier possède les fonctionnalités suivante:
///  * Générer une zone de jeu,
///  * Réinitialiser cette dernière,
///  * Permettre les interaction entre les joueur ou les intelligences artificielles via une unique
/// interface.
pub struct Engine<'a> {
    #[doc(hidden)]
    area : Area,
    #[doc(hidden)]
    player_one_interface : &'a RefCell<dyn Interface>,
    #[doc(hidden)]
    player_two_interface : &'a RefCell<dyn Interface>,
}

/// Zone de jeu
///
/// Ce dernier possède les fonctionnalités suivantes:
///  * Gérer l’ajout des jetons,
///  * Vérifier l’alignment de quatre jetons identiques.
#[derive(Clone)]
pub struct Area {
    #[doc(hidden)]
    area : [[State; AREA_ROWS]; AREA_COLS]
}

/// Interface entre les joueurs et le jeu.
///
/// Met en place les fonctions permettant le bon déroulement du jeu.
pub trait Interface {
    /// Demande au joueur de jouer
    ///
    /// # Arguments
    ///
    ///  * area : La référence vers l’aire de jeu actuel. Ce dernier peut être cloné pour tester
    /// des coups où être affiché par le joueur
    ///  * token : Le jeton joué par le joueur
    ///
    /// # Retour
    ///
    /// Une valeur correspondant à l’indice où placer le jeton. Ce dernier doit être compris entre
    /// 0 et `AREA_COLS-1`
    fn play(&mut self, area : &Area, token : State) -> usize;

    /// Donne le nom donné au joueur.
    ///
    /// # Retour
    ///
    /// Le nom du joueur sous la forme de chaine de caractères.
    fn name(&self) -> String;
}

impl Default for State {
    fn default() -> Self {
        State::NoToken
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sep = String::from_utf8(vec![0x2D;5*AREA_COLS+1]).unwrap();

        for col in 0..AREA_COLS {
            if let Err(e) = f.write_fmt(format_args!("|{: ^4}", col)) {
                return Err(e);
            }
        }

        if let Err(e) = f.write_fmt(format_args!("|\n{}\n", sep)) {
            return Err(e);
        }

        for row in 0..AREA_ROWS {

            for col in 0..AREA_COLS {
                if let Err(e) = f.write_fmt(format_args!("|{}", match self.area[col][AREA_ROWS - 1 - row] {
                    State::NoToken => "    ",
                    State::RedToken => " 🔴 ",
                    State::YellowToken => " 🟡 ",
                })) {
                    return Err(e);
                }
            }

            if let Err(e) = f.write_str("|\n") {
                return Err(e);
            }
        }

        if let Err(e) = f.write_str(&sep) {
            return Err(e);
        }

        return Ok(());
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FilledColumn => f.write_str("La colonne choisie est déjà pleine"),
            Error::InvalidColumn => f.write_str("La colonne choisie est invalide"),
            Error::NotAToken => f.write_str("L’élément fourni n’est pas un jeton"),
            Error::InvalidPlayerId(id) => f.write_str(format!("Le joueur {} n’existe pas. Seul 1 et 2 sont acceptés", id).as_str())
        }
    }
}

impl std::ops::Index<(usize, usize)> for Area {
    type Output = State;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return self.area.index(index.0).index(AREA_ROWS - 1 - index.1);
    }
}

impl std::ops::IndexMut<(usize, usize)> for Area {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        return self.area.index_mut(index.0).index_mut(AREA_ROWS - 1 - index.1);
    }
}

impl<'a> Engine<'a> {
    /// Crée un nouveau gestionnaire de jeux
    ///
    /// # Arguments
    ///
    ///  * `player_one_interface`: Interface vers le premier joueur, doit implémenter le trait `Interface`
    ///  * `player_two_interface`: Interface vers le second joueur, doit implémenter le trait `Interface`
    ///
    /// # Retour
    ///
    /// Une nouvelle instance de l’objet `Engine`
    pub fn new(player_one_interface : &'a RefCell<dyn Interface>, player_two_interface : &'a RefCell<dyn Interface>) -> Self {
        Self {
            area: Area { area: Default::default() },
            player_one_interface, player_two_interface
        }
    }

    /// Modifie le joueur identifié par `player_id` avec une nouvelle interface
    ///
    /// # Arguments
    ///
    ///  * `player_id`: L’identifiant du joueur (doit valoir soit 1 pour le joueur 1 ou 2 pour le
    /// joueur 2)
    ///  * `interface`: L’interface pour le joueur identifié par `player_id`
    ///
    /// # Retour
    ///
    /// Une erreur si l’identifiant du joueur est mauvais
    ///
    /// # Liste des erreurs possibles
    ///
    ///  * `InvalidPlayerId`: L’identifiant du joueur est invalide (doit valoir 1 ou 2)
    pub fn set_player(&mut self, player_id : usize, interface : &'a RefCell<dyn Interface>) -> Result<()> {
        if player_id != 1 && player_id != 2 {
            return Err(Error::InvalidPlayerId(player_id));
        }

        if player_id == 1 {
            self.player_one_interface = interface;
        } else {
            self.player_two_interface = interface;
        }

        return Ok(());
    }

    /// Fourni une référence vers la zone de jeu
    ///
    /// # Retour
    ///
    /// Une référence vers la zone de jeu
    pub fn get_disposition(&self) -> &Area {
        return &self.area;
    }

    /// Réinitialise la zone de jeu
    ///
    /// Une fois appelé, toutes les cases se retrouvent avec l’état sans jeton.
    pub fn reset(&mut self) {
        self.area.area = Default::default();
    }

    /// Joues une partie de puissance 4
    ///
    /// # Retour
    ///
    /// Une fois une partie finie, retourne un objet option valant None si le match est nul ou le
    /// nom du gagnant.
    ///
    /// Si une erreur se produit durant la partie, retourne l’erreur via l’objet `Error`. Ces
    /// erreurs sont émises en cas de problème avec les joueurs.
    ///
    /// # Liste des erreurs possibles
    ///
    ///  * `InvalidColumn` - L’identifiant de la colonne est invalide. Cette dernière doit être comprise entre 0 et `AREA_COLS-1`.
    ///  * `FilledColumn` - La colonne dont il est demandé une modification est déjà remplie.
    pub fn play(&mut self) -> Result<Option<String>> {
        let mut player = true;

        loop {
            let (token, interface) = if player {
                (State::YellowToken, self.player_one_interface)
            } else {
                (State::RedToken, self.player_two_interface)
            };

            let col = interface.borrow_mut().play(&self.area, token);

            match self.area.set_token(token, col) {
                Ok(t) => if t { return Ok(Some(interface.borrow().name())) },
                Err(e) => return Err(e),
            }

            if self.area.get_available_columns().is_empty() {
                return Ok(None);
            }

            player = !player;
        }
    }
}

impl Area {
    /// Récupère la liste des colonnes où on peut ajouter des jetons
    ///
    /// Vérifie la présence de jeton sur la dernière ligne pour chaque colonnes afin de construire
    /// une liste contenant seulement les indices de colonnes pouvant être joués
    ///
    /// # Retour
    ///
    /// La liste des colonnes non remplies
    pub fn get_available_columns(&self) -> Vec<usize> {
        let mut ret = Vec::with_capacity(AREA_COLS);

        for i in 0..AREA_COLS {
            if !self.is_filled_column(i).unwrap() {
                ret.push(i);
            }
        }

        return ret;
    }

    /// Vérifie si la colonne demandée est remplie
    ///
    /// # Arguments
    ///
    ///  * `column` : La colonne à vérifier, doit être comprise entre 0 et `AREA_COLS-1`
    ///
    /// # Retour
    ///
    /// Un booléen valant `true` si la colonne est remplie sinon `false` ou une erreur si la valeur
    /// donnée dans l’argument `column` est mauvaise.
    ///
    /// # Liste des erreurs possibles
    ///  * `InvalidColumn` - L’identifiant de la colonne est invalide. Cette dernière doit être comprise
    /// entre 0 et `AREA_COLS-1`.
    pub fn is_filled_column(&self, column : usize) -> Result<bool> {
        if column >= AREA_COLS {
            return Err(Error::InvalidColumn);
        }

        Ok(State::NoToken != self.area[column][AREA_ROWS-1])
    }

    /// Ajoute, si possible, un jeton dans l’aire de jeu
    ///
    /// # Arguments
    ///
    ///  * `token` : Le jeton. Doit avoir pour valeur `State::RedToken` ou `State::YellowToken`
    ///  * `column` : La colonne où ajouter le jeton, doit être comprise entre 0 et `AREA_COLS-1`.
    ///
    /// # Retour
    ///
    /// Un booléen valant `true` si le jeton ajouté permet la victoire sinon `false` ou une erreur
    /// si la valeur donné par `token` ne corresponds pas à celui d’un jeton ou si la valeur donnée
    /// par `column` est mauvaise.
    ///
    /// # Liste des erreurs possibles
    /// 
    ///  * `InvalidColumn` - L’identifiant de la colonne est invalide. Cette dernière doit être comprise
    /// entre 0 et `AREA_COLS-1`.
    ///  * `NotAToken` - L’état demandé ne correspond pas à celui d’un jeton.
    ///  * `FilledColumn` - La colonne dont il est demandé une modification est déjà remplie.
    pub fn set_token(&mut self, token : State, column : usize) -> Result<bool> {
        if let State::NoToken = token {
            return Err(Error::NotAToken);
        }

        if column >= AREA_COLS {
            return Err(Error::InvalidColumn);
        }

        if let Some(row) = self.find_available_row(column) {
            self.area[column][row] = token;

            Ok(self.check_victory_from(column, row))
        } else {
            Err(Error::FilledColumn)
        }
    }

    /// Vérifie si la zone de jeu est vide (premier coup)
    ///
    /// # Retour
    ///
    /// Retourne `true` si la zone de jeu est vide sinon `false`
    pub fn is_empty(&self) -> bool {
        for col in 0..AREA_COLS {
            if self.area[col][0] != State::NoToken {
                return false;
            }
        }

        return true;
    }

    fn check_victory_from(&self, col : usize, row : usize) -> bool {
        let token = self.area[col][row];

        let a = self.check_linear(token, col, true);
        let b = self.check_linear(token, row, false);
        let c = self.check_diagonal(token, col, row, true);
        let d = self.check_diagonal(token, col, row, false);

        return a || b || c || d
    }

    fn find_available_row(&self, column : usize) -> Option<usize> {
        for i in 0..AREA_ROWS {
            if let State::NoToken = self.area[column][i] {
                return Some(i);
            }
        }
        return None;
    }

    fn check_linear(&self, token : State, pos : usize, col : bool) -> bool {
        fn get(area : &[[State; AREA_ROWS]; AREA_COLS], pos : usize, i : usize, col : bool) -> State {
            if col {
                area[pos][i]
            } else {
                area[i][pos]
            }
        }

        let mut n = 0;

        let max = if col { AREA_ROWS } else { AREA_COLS };

        for i in 0..max {
            if token == get(&self.area, pos, i, col) {
                n += 1;
            } else {
                n = 0;
            }

            if n >= VICTORY_NUMBER {
                return true;
            }
        }

        return false;
    }

    fn check_diagonal(&self, token : State, col : usize, row : usize, decr : bool) -> bool {
        fn get(area : &[[State; AREA_ROWS]; AREA_COLS], origin : (usize, usize), i : usize, decr : bool) -> State {
            if decr {
                area[origin.0 + i][origin.1 + i]
            } else {
                area[origin.0 + i][origin.1 - i]
            }
        }

        let (origin, n_max) = if decr {
            let diff1 = usize::min(col, row);
            let diff2 = usize::min(AREA_COLS - col - 1, AREA_ROWS - row - 1);

            ((col - diff1, row - diff1), diff1 + diff2 + 1)
        } else {
            let diff1 = usize::min(col, AREA_ROWS - row - 1);
            let diff2 = usize::min(AREA_COLS - col - 1, row);

            ((col - diff1, row + diff1), diff1 + diff2 + 1)
        };

        if n_max < VICTORY_NUMBER {
            return false;
        }

        let mut n = 0;

        for i in 0..n_max {
            if token == get(&self.area, origin, i, decr) {
                n += 1;
            } else {
                n = 0;
            }

            if n >= VICTORY_NUMBER {
                return true;
            }
        }

        return false;
    }
}