# Formats de données (`data/`)

```
data/
  index.yaml                  liste des salles (id, nom, chemin, nombre d'entités/portes, taille de grille)
  rooms/room_NNN/
    room.yaml                 identité, tileset, musique, entités, portes, drapeaux, animations ...
    map.tmx                   carte Tiled isométrique (voir ci-dessous)
    blocks.png                blocs 16x16 de la salle (palette appliquée), 32 par ligne ; blocks_priority.png
    anim_N.png                images des blocs animés (eau...) ; entity_N_*.png : images d'entités propres à la salle
    preview.png               la salle entière rendue (contrôle) ; palette.yaml
  tilesets/                   tilesets d'aide des couches de hauteurs : height, floor_type, cell_flags, cell_extra (.png + .tsx)
  sprites/ graphics/ text/    copiés du moteur C++ (images d'entités, polices, dialogues, cutscenes, comportements hérités...)
```

## map.tmx
Carte isométrique (cellule 32x16). Une seule grille pour tout : les cellules de hauteurs (héritées de la ROM) et les
blocs graphiques, qui sont décalés l'un par rapport à l'autre selon la salle (`map.blocks.origin`).
Couches : `background`, `foreground` (blocs, gid = index du bloc + 1, 0 = vide) puis, cachées par défaut, les quatre
couches de la grille de hauteurs, à l'identique de la ROM, une cellule = 16 bits :

| couche        | bits du mot | contenu |
|---------------|-------------|---------|
| `height`      | 8..11       | hauteur (0..15) |
| `floor_type`  | 0..5        | 0 normal, 1-4 sens de porte, 5 escalier, 6 porte, 7 trou, 8 téléporteur, 0x0B/0x0C échelle, 0x0F ascenseur, 0x10 pointes, 0x19 marais, 0x1A/0x26/0x27 porte verrouillée, 0x29 lave, 0x2A-0x2D glace |
| `cell_flags`  | 12..15      | 0x4 fermée à tous (mur/vide), 0x2 fermée aux PNJ |
| `cell_extra`  | 6..7        | restrictions de la ROM (conservées telles quelles) |

Pour peindre une cellule dans Tiled : choisir le tileset d'aide correspondant ; le numéro est écrit sur la tuile.
Propriétés de la carte : `room`, `grid_origin_x/y`, `cell_offset`.

## Repères
- Coordonnées « monde » (entités, portes) : cellule du monde = index de la grille de hauteurs + `map.cell_offset` (12).
- Index de la grille de hauteurs (0,0) = première cellule de la ROM ; la grille TMX commence à `map.grid.origin`
  (négatif quand les blocs dépassent la grille de hauteurs). `map.blocks.origin` = position du bloc (0,0) dans ces index.
- Le bloc d'une cellule du `foreground` est dessiné 16 px à droite de la pointe de la cellule (décalage de tileset `<tileoffset x="32">`).
- Le `background` est décalé de 16 px de plus vers la droite (`offsetx="16"` sur la couche), comme dans le moteur C++
  (`DrawLayer(bg, 2)` : 2 unités de grille). Le `foreground` et les cellules de hauteurs sont la référence.

## room.yaml
`id`, `name`, `path` (nom complet de l'éditeur), `tileset`, `music`, `heights`, `fall_destination`, `climb_destination`,
`map` (fichier, grille, blocs), `entities` (id, type, position [x,y,z], orientation, vitesse, palette, `behaviour` hérité,
`script` (Lua, à venir), `dialogue`, `flags` hostile/solid/visible/gravity/pickable/talks), `warps` (zone [x,y,l,h] -> salle
d'arrivée + cellule), `animations`, `chests`, `characters`, et les sections héritées inchangées : `entity_flags`,
`tile_swaps`, `swap_flags`, `locked_doors`, `switches`.

## Vérifications
`lsctl verify-legacy` relit les couches TMX et compare avec `heightmap.csv`, `map_bg.csv`, `map_fg.csv` du moteur C++ :
0 différence sur les 816 salles. L'alignement dans l'application Tiled elle-même n'a pas été vérifié (pas d'écran) ;
`tools/render_tmx.py` reproduit sa règle de placement et donne le même rendu que le moteur.
