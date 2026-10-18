# LSA : Landstalker Asset (archive d'assets)

Un fichier `.lsa` contient plusieurs fichiers nommés dans un seul binaire. **Un LSA par sprite** : le YAML du sprite et
toutes ses images PNG sont dans la même archive, et rien d'autre.

| archive | contenu |
|---|---|
| `data/sprites/sprite_NNN.lsa` | `sprite.yaml` + ses images, rangées par pose (`walk/back_0.png`…) (sprite animé NNN : Nigel, coffre, monstres…) |
| `data/sprites/entities/entity_NNN.lsa` | `entity.yaml` (le type d'objet NNN : hitbox, PV, attaque, poses et images) + ses images (`idle/back.png`, `walk/front_2.png`…) |

Les noms dans une archive sont relatifs à elle-même (`sprite.yaml`, `idle/any.png`) ; le contenu des YAML est décrit dans `sprites_format.md`.

## Lecture : YAML + PNG, ou LSA
Le moteur lit les sprites des deux façons (`landstalker.ini`, section `[data]`) :

| `sprites =` | effet |
|---|---|
| `auto` (défaut) | un fichier à plat de `data/sprites/…` s'il existe (on édite, ça gagne), sinon l'archive du sprite |
| `files` | uniquement les fichiers à plat (YAML + liste de PNG, comme pendant le développement) |
| `lsa` | uniquement les archives (livraison : on peut supprimer les dossiers à plat) |

Le code demande toujours `sprites/sprite_000/sprite.yaml` ou `sprites/entities/entity_018/idle/any.png` ; le lecteur
(`assets.rs`, `DataFiles`) traduit en « archive + nom » et ouvre l'archive à la demande (une seule fois, mise en cache).
Les définitions d'objets sont les `entity.yaml` des dossiers `entities/entity_NNN/` ou des archives.

## Outil
```
lsctl lsa-pack-sprites data/sprites              # un .lsa par sprite et par type d'objet (relit tout, vérifie les crc32)
lsctl lsa-pack <dossier> <out.lsa>               # un dossier quelconque dans une archive
lsctl lsa-list data/sprites/sprite_000.lsa
lsctl lsa-unpack data/sprites/sprite_000.lsa dossier   # remet les fichiers à plat (édition, mods)
```
Le flux de travail : éditer les fichiers de `data/sprites/`, puis `lsa-pack-sprites` pour les archives de production.

## Format (petit-boutiste)
```
en-tête (32 octets)
   0  4  magic "LSA1"
   4  4  options (0)
   8  4  nombre d'entrées
  12  4  réservé (0)
  16  8  position de l'index
  24  8  taille de l'index
données  les fichiers les uns derrière les autres, bruts ou compressés en zstd
index    par entrée : u16 longueur du nom, nom (UTF-8, '/' comme séparateur), u8 méthode (0 brut, 1 zstd),
         u8 type (0 autre, 1 yaml, 2 png), u16 réservé, u64 position, u32 taille stockée, u32 taille d'origine,
         u32 crc32 du fichier d'origine
```
- Choix de conception : l'index est à la fin (une écriture en une passe) et non compressé (lecture directe, quelques dizaines de
  Ko) ; chaque fichier est compressé seul (zstd niveau 19), donc lisible seul ; les PNG, déjà compressés, restent bruts quand
  zstd ne fait pas gagner 5 % ; le crc32 détecte une archive abîmée.
- Les entrées sont triées par nom : deux constructions du même dossier donnent le même fichier.
- Le format sert pour toute arborescence : on pourrait y mettre les salles ou l'audio de la même façon.
- Code : `crates/ls_data/src/lsa.rs` (lecteur + écriveur + tests), 108 archives de sprites + 236 de types d'objets : 4,9 Mo → 1,8 Mo.
