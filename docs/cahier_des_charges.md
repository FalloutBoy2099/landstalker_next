# CAHIER DES CHARGES : PROJET "LANDSTALKER ENGINE NEXT-GEN"

## 1. Objectifs et Philosophie du Projet

* **Modernisation technique :** S'affranchir des contraintes matérielles de la Mega Drive (Assembleur 68k, limites de sprites hardware, registres de palettes restreints) tout en offrant une version enrichie et modernisée de *Landstalker* sans contrainte de fidélité à 100 %.
* **Approche hybride (Bevy + Overrides ciblés) :** Utiliser la puissance de l'écosystème Rust et de l'architecture ECS de Bevy pour toute l'infrastructure (assets, boucle de jeu, entrées, hot-reload, devtools) tout en écrivant du code sur-mesure pour les spécificités isométriques (rendu et physique 2.5D).
* **Approche orientée données (*Data-Driven*) :** Séparer strictement le code moteur (Rust), la logique de jeu (Lua) et les données de niveau (YAML / Formats binaires / TMX).
* **Confort de développement & Modding :** Permettre l'édition visuelle en temps réel via Tiled, le *hot-reloading* natif à chaud de tous les assets/scripts, et une architecture ouverte au modding.

---

## 2. Stack Technologique (Refondue)

* **Langage principal :** Rust (sécurité mémoire, concurrence et performances).
* **Moteur & Infrastructure :** Bevy (Architecture ECS, gestion des assets, fenêtrage, boucle de jeu, hot-reload, multi-plateforme).
* **Scripting :** Lua via `mlua` (avec support LuaJIT possible).
* **Données de cartes (Dev) :** TMX / Tiled via le crate `bevy_ecs_tiled` (gestion native de l'isométrique et du hot-reload).
* **Données de cartes (Prod) :** Formats binaires bruts (`.bin.zst`) via des `AssetLoader` Bevy custom avec compression Zstandard.
* **Interface DevTools / Debug :** `egui` (intégré avec Bevy via `bevy_egui`).
* **Audio :** Intégration audio native Bevy / `rodio` + crate spécialisé MOD/XM / MIDI (ex: `kira` ou `rustysynth`).

---

## 3. Architecture des Assets, Données & Modding Support

### A. Séparation des calques et des Rooms
* **Fichiers YAML de Room :** Contiennent l'identité de la pièce (ID, nom, musiques, flags globaux), la liste des entités (PNJ, coffres, déclencheurs) et les chemins vers les fichiers géographiques.
* **Géographie (Double format) :**
  * *Mode Dev :* Fichiers TMX/JSON éditables dans Tiled et importés via `bevy_ecs_tiled`.
  * *Mode Prod :* Fichiers binaires bruts optimisés (`.bin`) compressés avec Zstandard, décodés via un `AssetLoader` Rust dédié.

### B. Système de Modding Ouvert
* **Dossier de mods dédié (`/mods/`) :** Bevy charge les assets dans un ordre hiérarchique prioritaire. Tout fichier placé dans `/mods/[mod_name]/` peut surcharger un asset d'origine (textures, scripts Lua, maps TMX, sons) ou en ajouter de nouveaux sans altérer le jeu de base.

### C. Hiérarchie des Éléments Graphiques
```
[Graphics (Tiles/Atlas PNG)]
       │
       ▼
[TileSets (Propriétés des tuiles)]
       │
       ▼
[BlockSets (Blocs 3D isométriques + hauteur Z + collisions)]
       │
       ▼
[Rooms (YAML)] ──< contient la grille via BlockSets >── [Entities (PNJ/Objets)]
                                                               │
                                                     (pointent vers des)
                                                               │
                                                               ▼
                                                       [Sprites Animés]
```

---

## 4. Exemples de Structure de Données & Scripts

### Exemple A : Fichier YAML de définition d'une Room (`room_0104.yaml`)

```yaml
id: 104  
name: Massan - Inside Tree House  
theme: house_interior  

metadata:  
  music: bgm_house  
  flags: [indoor, no_magic]  

dimensions:  
  width: 16  
  height: 16  
  max_height: 8  

layers:  
  ground: assets/maps/raw/room_104_ground.bin  
  fringe: assets/maps/raw/room_104_fringe.bin  

entities:  
  - id: 1  
    type: chest  
    position: [5, 8, 2]  
    script: scripts/chests/standard.lua  
    content: { type: item, id: life_stock }  

  - id: 2  
    type: npc  
    name: Friday  
    position: [6, 8, 2]  
    script: scripts/rooms/massan_tree_house/friday.lua  

warps:  
  - area: [7, 15, 0, 2, 1]  
    target_room: 101  
    target_pos: [12, 3, 0]  
    fade: black  
```

---

### Exemple B : Fichier Script Lua d'Événement / PNJ (`friday.lua`)

```lua
local Entity = require("engine.entity")  
local UI = require("engine.ui")  
local Game = require("engine.game")  

local M = {}  

function M.on_load(entity)  
    print("Friday entre en scène dans la salle !")  
end  

function M.on_interact(entity, player)  
    if not Game.get_flag("met_friday_treehouse") then  
        UI.show_dialogbox({  
            "dialogs.friday_treehouse_01",  
            "dialogs.friday_treehouse_02"  
        })  
        Game.set_flag("met_friday_treehouse", true)  
        entity:jump_to({x = entity.x + 1, y = entity.y, z = entity.z}, 15)  
    else  
        UI.show_dialogbox({  
            "dialogs.friday_treehouse_03"  
        })  
    end  
end  

function M.on_update(entity, dt)  
end  

return M  
```

---

## 5. Moteur de Rendu Custom Isométrique & Rétro-HD

* **Système de Rendu Custom sur-mesure (Override Bevy) :** Implémentation d'un Render System sur-mesure sous Bevy pour gérer précisément le tri isométrique selon la formule $K = X + Y + Z$ et les déclencheurs de calques dynamiques (`z_layer_triggers`).
* **Shaders de Palette (Palette Swapping) :** Intégration via les matériaux 2D de Bevy ou une pass de rendu custom GPU pour le traitement des couleurs indexées (8-bit PNG) avec shaders WGSL.
* **Affichage Full HD / 16:9 & Cartes Étendues :** Support des résolutions modernes avec *Integer Scaling* et extension des grilles TMX/YAML pour combler les zones hors champ.
* **Pipeline Zéro-Clignotement (*Zero-Flicker*) :** Suppression des contraintes d'époque, gestion fluide de la semi-transparence.
* **Effets Post-Processing 2.5D :** Éclairages dynamiques 2.5D, ombres portées et effets de particules gérés dans l'ECS Bevy.

---

## 6. Physique et Collisions 2D.5 (Custom Engine)

* **Module Physique Custom (Override Bevy) :** Moteur de physique 2.5D dédié développé en Rust, combinant les boîtes de collision AABB 3D (sur $X, Y, Z$) et la grille de hauteur (Heightmap).
* **Gestion spatiale :** Déplacements sub-pixel, gravité, sauts, rebonds et gestion de l'élévation sur plateformes mobiles et caisses empilées.

---

## 7. Système de Combat Enrichi & Contrôles Analogiques

* **Gestion Analogique des Coups (Gâchettes L2/R2) :**
  * *Pression courte / Pression rapide :* Coup rapide, léger et immédiat (interruption).
  * *Pression maintenue / longue :* Attaque lourde chargée / Balayage de zone en arc de cercle couvrant les cases diagonales adjacentes avec un fort effet de recul (*Knockback*) pour se dégager d'un encerclement multi-adversaires.
* **Garde Active & Parade Parfaite (*Parry*) :** Orientation du bouclier pour bloquer les dégâts de face (réduction de 80%). Un timing précis de parade stoppe l'ennemi net et brise sa posture.
* **Refonte du Coup Sauté :** Attaque ascendante contre volants et piqué descendant avec onde de choc.
* **Jauge d'Équilibre (Stagger) :** Mécanique de posture gérée dans les composants Bevy.

---

## 8. Scripting Lua & Binding `mlua`

* **Intégration Rust/Lua :** Utilisation du crate `mlua` pour lier l'ECS Bevy à l'environnement Lua via une file d'attente d'événements (Event Queue) pour préserver les règles d'emprunt de Rust.
* **Isolation par Room / Entité :** Sandboxing léger par instance de script.
* **API exposée à Lua :**
  * Boîtes de dialogue (`UI.show_dialogbox`), qui reçoivent des clés i18n (`dialogs.friday_treehouse_01`) et jamais de texte en dur (voir §11).
  * Drapeaux de quêtes et sauvegarde (`Game.set_flag`, `Game.get_flag`).
  * Manipulation spatiale (`entity:move_to`, `entity:jump_to`, `target:apply_knockback`).

---

## 9. Interface Utilisateur (UI / UX)

* **Jauges Contextuelles Ennemies :** Affichage dynamique au-dessus des entités et barre de Boss globale.
* **HUD 16:9 Non-Intrusif :** Jauges de vie/endurance, sélecteur rapide d'objets aux gâchettes et mini-carte isométrique.
* **Menus & Textes :** Bulles de dialogue adaptatives avec portraits animés et inventaire ergonomique.

---

## 10. Système Audio

* **Musiques :** Prise en charge des formats modulaires (MOD/XM) ou MIDI+SF2 via des crates dédiés.
* **Effets Sonores (SFX) :** Échantillons PCM WAV/FLAC déclenchés par les événements ECS.

---

## 11. Localisation (i18n) & Expérience Développeur (DX)

* **Système i18n Data-Driven (YAML) :** Séparation stricte du code et des textes via des dictionnaires multilingues au format YAML (`assets/i18n/[lang]/`). Centralisation par domaine (dialogues, items, équipements, interfaces).
* **Architecture Zéro-Hardcode :** Les scripts Lua et le code source Rust ne manipulent que des clés textuelles (ex : `items.life_stock`, `dialogs.friday_treehouse_01`), garantissant une traduction et un support multi-langues fluide (français, anglais, japonais, etc.).

Exemple de dictionnaire (`assets/i18n/fr/dialogs.yaml`) :

```yaml
friday_treehouse_01:
  speaker: "Friday"
  text: "Nigel ! Attends-moi ! Il ne faut pas oublier le coffre !"
friday_treehouse_02:
  speaker: "Nigel"
  text: "J'arrive, j'arrive... Qu'est-ce qu'il y a dedans ?"
```

* **Outils de développement :**

* **Hot-Reloading Natif :** Pris en charge par le système d'assets de Bevy pour la mise à jour à chaud des scripts Lua, des maps TMX et des fichiers YAML sans recompiler.
* **Outils DevTools (`egui` / `bevy_egui`) :**
  * Console d'exécution Lua interactive en jeu.
  * Inspecteur d'entités ECS (modification des composants en direct).
  * Affichage filaire (*wireframe*) des hitboxes AABB 3D et des hauteurs Z.
* **Localisation (i18n) :** Fichiers YAML gérés par des loaders Bevy.