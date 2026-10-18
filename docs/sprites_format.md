# Sprites : poses et directions

Chaque type d'objet (`entity_NNN`) et chaque sprite animé (`sprite_NNN` : Nigel, le coffre…) décrit ses images par **poses**, chacune vue
sous une ou plusieurs **vues**, chaque vue étant une liste d'images. La direction de l'entité choisit la vue et le miroir.

```
data/sprites/
  sprite_000/sprite.yaml  +  idle/back_0.png  walk/front_3.png  sword/back_1.png ...    (Nigel)
  sprite_036/sprite.yaml  +  open/0.png ...                                              (le coffre)
  sprite_NNN/sprite.yaml  +  les images exportées (poses anim_00, anim_01... en vue any)
  entities/entity_004/entity.yaml  +  idle/any.png idle/back.png idle/front.png walk/back_0.png attack/... hurt/...
```
Chaque dossier est aussi une archive : `sprite_000.lsa`, `entities/entity_004.lsa` (voir `lsa_format.md`). Les images d'une entité propres
à une salle (palette de la salle) sont dans le dossier de la salle (`rooms/room_000/entity_7_back.png`), décrites dans `room.yaml`.

## entity.yaml
```yaml
sprite: 19                     # le sprite d'où viennent les images
hitbox_base: 8                 # demi-côté de la boîte (seizièmes de case) et hauteur
hitbox_height: 32
enemy: true                    # combat : health, defence, attack, gold_drop
health: 4
defence: 5
attack: 2
gold_drop: 2
poses:
  idle:                        # la pose d'un objet immobile (idle), qui marche (walk), attaque (attack), est touché (hurt)
    any:   [ ... ]             # une image qui sert pour toutes les directions
    back:  [ ... ]             # vue de dos (NE, NW)
    front: [ ... ]             # vue de face (SE, SW)
  walk:
    back:
      - {file: "walk/back_0.png", w: 32, h: 48, ax: 16, ay: 40}   # fichier relatif au dossier, taille, pixel posé sur la position
      - {file: "walk/back_1.png", w: 48, h: 48, ax: 24, ay: 40}
```
Une pose qui n'a pas de vue pour une direction est ignorée pour elle (un objet qui n'a pas de pose `attack` garde son image au repos).

## sprite.yaml (Nigel, coffre…)
Les notes de l'exporteur (`sprite:`, `entities:`, `palette:`) sont conservées ; `poses:` remplace les anciens `frames` et `animations`.
Poses de Nigel : `idle, walk, lift, carry_walk, jump, carry_jump, hurt, sword, hold_item (vue any), death`. Le coffre : `open` (fermé,
s'ouvre, ouvert). Les autres sprites exportés (non utilisés directement par le jeu) : `anim_NN`, vue `any` (leur convention
dos / face d'origine, par parité des numéros, n'est pas déduite).

## Directions
Les quatre directions de la vue isométrique sont NE, SE, SW, NW (orientation d'une entité : 0 NE, 1 SE, 2 SW, 3 NW ; Nigel regarde 0 NW,
1 NE, 2 SE, 3 SW). Par défaut, comme sur la console :

| direction | vue | miroir |
|---|---|---|
| NE | back | non |
| NW | back | oui |
| SE | front | oui |
| SW | front | non |

Un sprite peut changer cette table, et fournir un vrai dessin pour une direction :
```yaml
directions:                    # facultatif ; ce qui n'est pas donné garde la valeur par défaut
  NW: {view: back, mirror: false}
poses:
  idle:
    sw: [ {file: "idle/sw.png", ...} ]    # dessiné pour SW seulement (jamais miroité) ; aussi ne, se, nw
```
Ordre de recherche (`Views::pick`, `crates/ls_data/src/sprites.rs`) : l'image dessinée pour la direction (`ne/se/sw/nw`), sinon la vue que
donne la table (`back` / `front`), sinon `any` ; le miroir est celui de la table (sauf pour une image dessinée pour la direction).

## Migration
`tools/migrate_sprites.py [data]` convertit l'ancien format (`entities.yaml` en une ligne par type avec `default_file`, `back_file`,
`walk_back "fichier|w|h|ax|ay;…"`…, `frames` + `animations` à parité de numéros, `images:` des salles) vers celui-ci : idempotent,
appelé à la fin de `tools/convert_data.py` (les exports du moteur sont dans l'ancien format). Les images déménagent dans les dossiers de
pose. Contrôle : `lsctl check` lit tous les `entity.yaml` / `sprite.yaml` et vérifie que chaque image existe.
