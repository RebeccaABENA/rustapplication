use std::io;


#[derive(Debug, Clone)]  
struct CompteBancaire {
    nom: String,
    solde: f32,
}


impl CompteBancaire {
    fn afficher_solde(&self) {
        println!("Solde de {} : {:.2} €", self.nom, self.solde);
    }

    fn retrait(&mut self, montant: f32) {
        if montant > self.solde {
            println!("Fonds insuffisants pour {}", self.nom);
        } else {
            self.solde -= montant;
            println!("Retrait effectué. Nouveau solde de {} : {:.2} €", self.nom, self.solde);
        }
    }

    fn depot(&mut self, montant: f32) {
        if montant < 0.0 {
            println!("Desolé on ne peut pas déposer un montant négatif ");
        } else {
            self.solde += montant;
            println!("Dépôt réussi. Nouveau solde : {:.2} €", self.solde);
        }
    }

    fn renommer(&self, nouveau_nom: &str) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom.to_string(),
            solde: self.solde,
        }
    }
}

fn main() {
    
    let mut comptes = vec![
        CompteBancaire { nom: "Alice".to_string(), solde: 1200.0 },
        CompteBancaire { nom: "Denzel".to_string(), solde: 800.0 },
        CompteBancaire { nom: "Kelvin".to_string(), solde: 500.0 },
    ];

    let options = [
        "Afficher tous les soldes",
        "Retrait",
        "Dépôt",
        "Renommer un compte",
        "Quitter",
    ];

    loop {
        println!("\n=== Menu Principal ===");
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        println!("Votre choix :");
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();

        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrée invalide");
                continue;
            }
        };

        match choix {
            1 => {
                println!("\n--- Soldes des comptes ---");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}. {} : {:.2} €", i + 1, compte.nom, compte.solde);
                }
            }
            2 => {
                let index = selectionner_compte(&comptes);
                if let Some(i) = index {
                    println!("Montant à retirer : ");
                    let montant = lire_montant();
                    comptes[i].retrait(montant);
                }
            }
            3 => {
                let index = selectionner_compte(&comptes);
                if let Some(i) = index {
                    println!("Montant à déposer : ");
                    let montant = lire_montant();
                    comptes[i].depot(montant);
                }
            }
            4 => {
                let index = selectionner_compte(&comptes);
                if let Some(i) = index {
                    println!("Nouveau nom : ");
                    let mut nouveau_nom = String::new();
                    io::stdin().read_line(&mut nouveau_nom).unwrap();
                    comptes[i] = comptes[i].renommer(nouveau_nom.trim());
                    println!("Compte renommé ");
                }
            }
            5 => {
                println!("À bientôt !");
                break;
            }
            _ => println!("Option invalide."),
        }
    }
}


fn lire_montant() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Entrée invalide. Valeur mise à 0.");
            0.0
        }
    }
}


fn selectionner_compte(comptes: &Vec<CompteBancaire>) -> Option<usize> {
    println!("Sélectionnez un compte :");
    for (i, compte) in comptes.iter().enumerate() {
        println!("{}. {}", i + 1, compte.nom);
    }

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    match choix.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= comptes.len() => Some(index - 1),
        _ => {
            println!("Compte invalide.");
            None
        }
    }
}
