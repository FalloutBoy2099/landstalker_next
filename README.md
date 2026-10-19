# landstalker_next

Refonte de Landstalker en Rust + Bevy (cahier des charges : `docs/cahier_des_charges.md`).
Le moteur C++ (`../landstalker_engine`) reste la référence de comportement ; ses fichiers exportés sont la source des données.

## État
- [x] **Données** : conversion de tous les fichiers de jeu (816 salles) en YAML + TMX (`tools/convert_data.py`) -> `data/`.
- [x] **Lecture en Rust** : `crates/ls_data` (types, TMX, cellules) et `crates/lsctl` (outil en ligne de commande).
- [ ] Rendu isométrique Bevy, physique 2.5D, scripts Lua (`mlua`), éditeurs.

## Commandes
```
python tools/convert_data.py                 # (re)génère data/ depuis ../landstalker_engine/assets
cargo run --release -p lsctl -- check        # 816 salles chargent, warps/fichiers cohérents
cargo run --release -p lsctl -- verify-legacy  # les cartes converties == les fichiers du moteur C++, case par case
cargo run --release -p lsctl -- room 27      # résumé d'une salle
python tools/render_tmx.py data/rooms/room_018 out.png   # rendu d'une salle depuis son TMX (contrôle)
```
Format des fichiers : `docs/data_formats.md`.

## Son, configuration, monde (état)
- **Musique en VGM** (`docs/vgm.md`) : `data/audio/vgz/music_XX.vgz` (8 Mo pour les 40 morceaux) est le journal des écritures du driver son
  du jeu dans le YM2612 et le SN76489 ; le jeu rejoue ces puces en direct (portage Rust de Nuked-OPN2, identique bit à bit au moteur
  C++), boucle sans coupure. `tools/render_vgm.py` fabrique les fichiers (moteur C++ + `lsctl evt-to-vgm`, qui trouve la boucle).
  `landstalker.ini` : `[audio] music_format = auto | vgm | ogg`.
- `tools/render_sounds.py` rend une fois les sons du jeu (émulation Z80 + YM2612 du moteur C++) en `data/audio/` rangé par format :
  `flac/sfx_XX.flac` (bruitages), `ogg/music_XX.ogg` + `ogg/music_XX.yaml` (début et fin de la boucle, en échantillons ; `loop: false` pour un jingle)
  et `vgz/music_XX.vgz` (journaux VGM de `render_vgm.py`). Les boucles sont détectées automatiquement : corrige le `.yaml` si une boucle sonne mal.
- `landstalker.ini` (créé au premier lancement, à la racine du projet) : fenêtre, volumes, touches (clavier), manette,
  options de debug. Tout se règle dedans ; les variables d'environnement `LS_*` restent des aides de test.
- Tuiles animées (eau, lave, cascades), échanges de tuiles selon les drapeaux (portes qui s'ouvrent, ponts), règles de
  visibilité des entités par drapeaux, puits (chute vers la salle du dessous), échelles.

## Combat (état)
- Épée (touche `X` / `Entrée`, bouton X de la manette) : le coup part à 0,10 s, touche devant Nigel, repousse l'ennemi ; Nigel ne
  marche pas pendant le coup. Formules de dégâts de l'original (`game.rs` : armure, bonus aléatoire, défense).
- Ennemis (`data/scripts/ai.lua`, port de `RoomView::UpdateEnemies` du moteur C++) : patrouille, vue en rectangle, poursuite par
  axes, bond (orcs, lézards, ninjas…), flottants (fantômes), sauteurs (champignons), rebondisseurs (bulles), mimiques, boules
  piquantes ; images « attaque » et « blessé » de chaque ennemi ; mort avec clignotement puis butin (sac d'or ou objet,
  `data/enemies.yaml`). Un ennemi tué ne revient pas ; un ennemi blessé garde ses points de vie.
- Nigel : points de vie en 1/256 (4 au départ, +1 par Life Stock), recul et clignotement (1 s d'invulnérabilité) après un coup,
  animation de mort puis `GAME OVER` (saut ou épée pour repartir). Barre de vie / or / EkeEke en haut à gauche.
- Sols : pointes (1), marais (1), lave (5) blessent (bottes de fer / ignifugées protègent) ; la glace entraîne Nigel (bottes à
  crampons : non). Objets au sol (type >= 0xC0) ramassés au contact.
- **Boss** (`ai.lua`, port de `UpdateBoss` du moteur C++) : Ifrit (garde ses distances, tire jusqu'à trois boules de feu), Gola (anneaux de
  boules de feu dans les quatre directions), Nole (marche sur Nigel et lance des haches qui reviennent), Zak (boomerangs), Mir (petites boules
  de feu), les toupies (foncent sur Nigel). Les projectiles filent le long des axes de la grille, s'arrêtent aux murs (un boomerang qui
  revient les traverse), blessent Nigel à l'impact (1/2 de l'attaque du boss) et occupent des lignes d'entités de réserve de la salle
  (`PROJECTILE_SLOTS` = 16 : dessinés comme n'importe quelle entité). La mort d'un boss dure 1,8 s, avec une explosion et un tremblement d'écran.
- **Maladies** : poison (1 PV toutes les 2 s de marche, 8 fois), confusion (commandes inversées), paralysie (20 s, ou on se débat avec l'épée :
  1 chance sur 3, puis 1 sur 2, puis sûr), malédiction (plus d'épée). Les vers et les champignons les donnent (40 % des coups). Nigel prend une teinte
  (vert, jaune, bleu, violet), la barre du haut les nomme ; les objets de soin les guérissent.
- **Inventaire** (Tab, bouton Y de la manette) : liste des objets, haut / bas pour choisir, action pour utiliser, menu ou attaque pour fermer ; le jeu
  est figé. EkeEke (aussi sur `E`) : rend la moitié de la vie max (10 points au moins). Clé : ouvre la porte verrouillée sur laquelle Nigel se tient
  (cases 26 + n, ou portes latérales 38 / 39). Épées, armures (réduisent les dégâts), bottes (de fer : pointes ; ignifugées : lave ; de soin ; à crampons :
  glace) : on les met ou on les enlève. Pierre d'idole : ouvre la porte du temple du marais (salle 433).
- **Magie** (la « magie » de Landstalker est celle des épées) : avec une épée équipée, la jauge de charge (en haut) monte de 32 par image à
  60 Hz, soit 6,7 s pour la remplir, et chaque coup d'épée la vide. Un coup à jauge pleine est magique : les dégâts sont multipliés
  (Épée magique ×1,75, de glace ×1,5, de foudre ×2, de Gaia ×1,25, `MagicSwordBoost` de l'original) ; l'épée de glace lance en plus une vague
  qui traverse les ennemis (8 cases), l'épée de Gaia blesse tous les ennemis de la salle. Le mécanisme est celui du désassemblage
  (`SwordSwing`, `IncrementSwordCharge`, `GaiaEffect`) ; les images des effets (l'original utilise des sprites) sont remplacées par des étincelles
  de la couleur de l'épée. Pas portés : les anneaux (Vénus, Saturne) qui accélèrent la charge, la Statue d'or, les effets sur le comportement des ennemis.
- **Boutiques, auberges, églises** (`shops.rs`, `data/shops.yaml` + `data/items.yaml`) : dans une boutique, les marchandises sont sur le comptoir ;
  face à l'une, l'action propose de l'acheter (prix de base × marge de la boutique / 16) avec une boîte de dialogue oui / non (gauche / droite, action pour
  valider, attaque pour refuser). L'aubergiste fait passer la nuit (vie pleine, 20 × marge / 16 pièces). Le prêtre guérit les maladies (le prix de la nuit
  par maladie) ou enregistre l'aventure : après la mort, Nigel se réveille dans cette église (rien n'est écrit dans un fichier : pas de sauvegarde encore).
- **Dialogues** (`dialogue.rs`, `data/dialogues/` (un fichier YAML par personnage), port de `Dialogue.h`) : face à un personnage qui parle, l'action ouvre sa conversation. Elle suit
  l'avancement de l'histoire (progression et drapeaux : `progress`, `set_flag_on_talk`, `check_flag`), donne des objets, pose des questions oui / non
  dont la réponse décide de la suite. Les actions de scénario (cutscenes) qu'une conversation demande sont notées dans la console (`[dialogue]`) mais pas jouées.
- **Sauvegarde** (`save.rs`) : `F5` enregistre, `F9` charge (`landstalker.sav` à la racine du projet : salle, position, or, objets, vie,
  équipement, maladies, progression, drapeaux d'histoire, coffres ouverts, ennemis tués et leur vie, église). Le prêtre enregistre aussi
  (« Your adventure is recorded »). Touches modifiables dans `landstalker.ini` (`save`, `load`).
- **Cutscenes** (`cutscene.rs`, les scènes sont des scripts Lua (`story(n, { say(9), ... })`, `data/scripts/rooms/room_NNN.lua` pour celles d'une room, `data/scripts/cutscenes/` pour les autres et les `playback_seq`) ; l'intro (`intro.track`, `intro_strings.yaml`) est dans `data/scripts/intro/` ; les crédits sont `data/i18n/<langue>/credit.txt`, port de `TriggerAction` / `ContinueCutscene`
  du moteur C++) : les scripts des personnages (`cutscene(e, n)` en Lua, avec la règle « une scène qui attend un drapeau ne se joue qu'une fois ») et les
  conversations demandent une action ; elle déroule ses opérations : textes (cases, attente automatique, question oui / non qui choisit la suite), drapeaux,
  sons, échanges de tuiles, personnages qui apparaissent / disparaissent / deviennent hostiles, cases bloquées, changement de salle derrière un fondu, effets
  (fondu au noir, éclairs), Friday la fée qui apparaît, flotte et repart, commandes retirées à Nigel (il ne bouge que par l'entrée enregistrée, `playback`),
  légendes de l'intro, générique. Les scripts et le jeu s'arrêtent pendant un fondu ou un texte, comme dans le moteur C++. L'évanouissement (action 0, salle 426)
  fait glisser Nigel dans le lit de la maison du maire (salle 593) avec Friday. Vérifié : 426 → évanouissement → 593 → actions 28 et 29 → Nigel libre ;
  `LS_NEWGAME=1` lance l'intro (salle 139, légendes, scènes des salles 139 à 142 observées), X ou Entrée la passe (salle 148, texte avec Friday).
  Pas portés : le fondu d'apparition des personnages (seulement secousse, grondement et étincelles), la position exacte de Friday (approchée).
- Pas encore : séquences de boss propres à l'original (le moteur C++ n'en a pas non plus), écran titre.
- Écran titre (`title.rs`, port de TitleScreen.h / Menus.h) : logo Climax (fondu, 3,8 s), puis l'image du titre apparaît en balayage depuis la gauche, musique 0x10 ; menu NEW GAME (intro, salle 139) / CONTINUE (charge `landstalker.sav`, grisé sans sauvegarde) / QUIT. Une touche passe le logo. Échap en jeu ouvre le menu pause : RESUME / LOAD GAME / QUIT TO TITLE / QUIT (Échap ne quitte plus directement). Lancer avec un numéro de salle (`landstalker 18`) ou `LS_NOTITLE=1` saute le titre ; `LS_TITLE=1` le force, `LS_TITLE_T=s` avance l'écran titre de `s` secondes (captures).

- Menu game over (`title.rs`, port de ShowGameOver) : à la mort, l'animation de chute puis le menu CONTINUE (réapparition à l'église où l'aventure a été enregistrée, sinon sur place, vie pleine) / LOAD GAME (grisé sans sauvegarde) / QUIT TO TITLE. Test : `LS_DIE=s` tue Nigel à `s` secondes.
- Intro enregistrée sur la ROM (`track.rs`, `data/scripts/intro/intro.track`, `tools/emuhawk/`) : un nouveau jeu rejoue l'intro de l'original image par image (salles 139 à 143 : positions de Nigel et de chaque entité, fondus, légendes, boîtes de message, dialogues d'Owner/Kayla/Friday lancés aux images mesurées), avec les bandes noires de l'original (lignes 24 à 184 sur 224), puis l'aventure commence en 148. Sans le fichier `.track`, l'ancienne intro par scripts est jouée. Mesures de la ROM appliquées au jeu : 60 ticks de script par seconde, marche à 7,5 cellules/s, saut de 1,75 unité en 24 images, Nigel à 85 pixels du haut de l'écran, échelle 4 (320x224 -> 1280 px). La caméra peut se centrer sur un PNJ (`camera_focus(i)` en Lua, automatique pour une scène lancée par un boss).
- Textes en clés i18n (`i18n.rs`, `data/i18n/<langue>/*.yaml`, cahier des charges §11) : le code Rust et les scripts Lua ne manipulent que des clés (`ui.title.new_game`, `messages.shop.ask_buy`, `items.life_stock`, `cutscenes.t253`, `dialogs.c0_progress_0_node_m0`) ; `i18n::t(clé)` / `tf(clé, valeurs)` / `pages(clé)`, `t(clé)` en Lua. Une clé absente de la langue retombe sur l'anglais, puis s'affiche telle quelle. Langue : `[game] language` de landstalker.ini ou `LS_LANG=fr` (l'interface et les messages sont traduits en français, les textes de la ROM restent en anglais tant que le dictionnaire français n'existe pas). Les textes de la ROM ont été sortis des tables exportées par `tools/export_i18n.py` (les fichiers de données ne gardent que des clés) ; `ui.yaml` et `messages.yaml` sont écrits à la main. Les légendes de l'intro sont des images de la police d'origine (hors dictionnaire).
- Aides de test supplémentaires : `LS_LOG_ENTS=n` (positions de Nigel et des entités toutes les n images, au format de la RAM de l'original), `LS_TRACK_AT=image` (démarre l'intro enregistrée à cette image), `LS_DIE=s` (mort de Nigel), `LS_CAMERA_FOCUS=n`.
- Aides de test : `LS_HP=n` (vie max), `LS_ITEMS="id:n,..."` (objets), `LS_NEWGAME=1`, `LS_LOG_CUTSCENE=1`, `LS_GOLD=n`, `LS_LOG_DIALOGUE=1`, `LS_SWORD=1..4` (épée portée), `LS_CHARGE=1` (jauge pleine au départ), `LS_STATUS=poison,confusion,paralysis,curse`, `LS_WAKE=1` (réveille les ennemis endormis par un scénario : boss), `LS_LOG_COMBAT=1`, `LS_DBG_FLOOR=1`, `lsctl find-floor <type>` (salles avec des cases de ce sol).

## Sprites : fichiers ou archive LSA
`data/sprites/` (YAML + PNG) ou un `.lsa` par sprite (`sprite_NNN.lsa`, `entities/entity_NNN.lsa`, voir `docs/lsa_format.md`) : `lsctl lsa-pack-sprites data/sprites`.
Les images sont décrites par **poses et directions** (idle, walk, attack, hurt, sword… vues `back` / `front` / `any`, miroir selon la direction) : `docs/sprites_format.md`. `tools/migrate_sprites.py` convertit l'ancien format (appelé par `convert_data.py`) ; `lsctl check` vérifie que toutes les images existent.
Choix dans `landstalker.ini`, `[data] sprites = auto | files | lsa`.
