# Environment

## Why

To start programming on this project, a few needs to be followed to unsure a successfull build.

## Global

### Étape 1: Créer le fichier `.git/hooks/pre-commit`

Dans le répertoire de ton projet, crée le fichier `pre-commit` dans le dossier `.git/hooks/` s'il n'existe pas déjà :

```bash
touch .git/hooks/pre-commit
```

### Étape 2: Ajouter le contenu du script

Ouvre ce fichier et ajoute les lignes suivantes pour exécuter `cargo fmt`, `cargo clippy` et `cargo check` avant chaque commit :

```bash
#!/bin/sh

# Exécuter cargo fmt pour formater le code
cargo fmt -- --check

# Exécuter cargo clippy pour analyser le code
cargo clippy --all-features --workspace -- -D warnings

# Exécuter cargo check pour vérifier que le code compile
cargo check --all
```

Ces commandes s'assureront que le code est formaté, sans avertissements et qu'il compile correctement avant chaque commit.

### Étape 3: Rendre le script exécutable

Assure-toi que le fichier a les permissions d'exécution en utilisant la commande suivante :

```bash
chmod +x .git/hooks/pre-commit
```

---

## 2. Configurer `Cargo.toml` pour faciliter l’utilisation du script

Dans ton fichier `Cargo.toml`, il n'y a pas de mécanisme intégré pour lier directement des hooks Git, mais tu peux t'assurer que les outils nécessaires comme `clippy` ou `rustfmt` sont présents.

Voici un exemple de ce à quoi pourrait ressembler ton fichier `Cargo.toml` avec ces outils :

```toml
[dev-dependencies]
# Ces dépendances sont uniquement utilisées lors des vérifications
clippy = "0.1"
rustfmt = "1.0"
```

Cela garantit que les outils nécessaires seront disponibles lorsque les commandes seront exécutées.

---

## Webserver

If running with WSL, create a ".wslconfig" file in your $User directory in windows and paste

```
[wsl2]
networkingMode=mirrored
```

Now for everyone.

- Install the rust toolchain 1.87.0-nightly
- Install cargo
- Follow this [tutorial](https://dioxuslabs.com/learn/0.6/getting_started/#) to get dioxus.
- Install DIoxus VSCode extension
- Cd into the webapp repo
- Use the command dx serve --addr 0.0.0.0 command

## embedded

To run the embedded part and install it on ESP32.

```
sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
```

Then, globally follow this :

- https://github.com/esp-rs/rust-build?tab=readme-ov-file

- Install all needed toolchain whith the command :

- Install he template generator : `cargo install cargo-generate`
- Install esp-idf :
  - `git clone --recursive https://github.com/espressif/esp-idf.git`
  - `cd esp-idf`
  - `./install.sh`
  - Add `export IDF_PATH=~/esp-idf` in bashrc.
