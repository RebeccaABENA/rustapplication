use std::io;

fn main() {
    // On crée un "compte" de départ
    let mut solde:f32 = 10000.0; // le solde du compte
    let mut comptes = vec!["Alice", "Bob", "Kevin"]; // liste de comptes

    // Menu
    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    println!("=== TP1 : Mini Banque ===");

    loop {
        println!("\nMenu:");
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        println!("Entrez votre choix :");
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrée invalide. Veuillez entrer un numéro.");
                continue;
            }
        };

        if choix == 0 || choix > options.len() {
            println!("Choix hors limite.");
            continue;
        }

        match choix {
            1 => {
                println!("Votre solde actuel est : {:.2} €", solde);
            }
            2 => {
                println!("Entrez le montant à retirer :");
                let mut montant = String::new();
                io::stdin().read_line(&mut montant).expect("Erreur de lecture");
                let montant: f32 = match montant.trim().parse() {
                    Ok(m) => m,
                    Err(_) => {
                        println!("Montant invalide.");
                        continue;
                    }
                };

                if montant > solde {
                    println!("Fonds insuffisants.");
                } else {
                    solde -= montant;
                    println!("Retrait effectué. Nouveau solde : {:.2} €", solde);
                }
            }
            3 => {
                println!("Liste des comptes :");
                for (i, nom) in comptes.iter().enumerate() {
                    println!("{}. {}", i + 1, nom);
                }
            }
            4 => {
                println!("Au revoir !");
                break;
            }
            _ => {
                println!("Option non reconnue.");
            }
        }
    }
}
