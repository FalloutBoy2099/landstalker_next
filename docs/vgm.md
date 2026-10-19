# Musique : VGM (YM2612 + SN76489)

La musique de la Mega Drive n'est pas un fichier audio : le driver son du jeu (un programme Z80 de 8 Ko) lit la partition dans la
ROM et **écrit dans les registres** de deux puces : le YM2612 (synthèse FM, 6 voies, plus un DAC pour les échantillons) et le
SN76489 (PSG : 3 carrés + un bruit). Le **VGM** (Video Game Music) est le journal de ces écritures, avec le temps entre elles.
Rejouer un VGM, c'est refaire tourner les puces : rien n'est approximé (ce que ne peuvent pas faire MIDI ou MOD, qui ne savent
pas décrire des instruments FM).

## Ce que le jeu lit
`data/audio/vgz/music_XX.vgz` (XX = numéro du morceau, en hexadécimal : table de `landstalker_disasm/.../sounds.inc`). Un `.vgz` est
un `.vgm` compressé avec gzip (le moteur lit aussi un `.vgm` non compressé). Les effets sonores restent des `sfx_XX.flac`.
`landstalker.ini`, `[audio] music_format = auto | vgm | ogg` : `auto` joue le VGM s'il existe, sinon l'OGG (`music_XX.ogg`).

Le morceau n'est pas « décodé » : les puces tournent **en direct** dans le thread audio, par blocs de 512 échantillons (un
centième de seconde ; le coût est de quelques pour cent d'un cœur), exactement comme sur la console. Rien à préparer au
changement de salle, pas de mémoire (un OGG décodé de 65 s pèse 14 Mo), et une boucle est vraiment infinie : quand un tour va
finir, les écritures du suivant sont mises en file, les puces ne s'arrêtent jamais, donc le son de la fin d'un tour s'éteint sur le
début du suivant, comme dans le jeu.

## Les puces (`crates/ls_data/src/ym2612.rs`, `vgm.rs`)
- **YM2612** : portage en Rust de Nuked-OPN2 (Nuke.YKT, LGPL 2.1+), le même émulateur que le moteur C++ ; il suit le source C ligne
  par ligne (mêmes noms de registres et de variables, largeurs d'entiers conservées).
- **SN76489** : celui du moteur C++ (carrés, registre de bruit).
- Le mixage est celui de `SoundEngine::Render` du moteur (gain FM, PSG, filtre passe-haut du DAC) : **bit à bit identique** à ce que
  rend le moteur C++ (vérifié : 30 s du morceau 01, écart maximal 0 sur 1,6 million d'échantillons ; voir « Vérifier »).
- Fréquence de sortie : 53267 Hz (celle de la puce), stéréo.

## Fabriquer les fichiers : `tools/render_vgm.py`
1. Le moteur C++ lance le driver Z80 sur la ROM (`LS_EVENT_DUMP="id:secondes:fichier.evt"`) et note chaque écriture dans le YM2612 ou
   le PSG avec son instant (en horloges de la puce : 24 par échantillon).
2. `lsctl evt-to-vgm morceau.evt music_XX.vgz [--once]` cherche où le morceau se met à se répéter : le driver est déterministe, une
   boucle rejoue les mêmes écritures aux mêmes distances (à quelques horloges près : la gigue du Z80). Le point de boucle est le
   (début, période) dont la répétition remonte le plus loin et contient au moins deux tours. Un morceau qui joue des échantillons
   (DAC, registre 0x2A) est cherché sans eux : leur nombre change d'un tour à l'autre. `--once` : un jingle (pas de boucle) ; on
   coupe où le driver, fini, se met à écrire toujours les mêmes registres.
   Si aucune boucle n'est trouvée dans un morceau qui devrait en avoir une, `render_vgm.py` enregistre plus longtemps (jusqu'à 10 min :
   la durée de `music_XX.json`, tirée de l'audio, est parfois fausse) ; un morceau sans boucle est coupé 6 s après sa dernière note.
3. Le fichier est un VGM 1.50 standard : n'importe quel lecteur VGM (VGMPlay, foobar2000 avec un plugin…) le joue.

Temps en secondes de 44100 Hz (le standard) : les écritures sont arrondies au 44100e de seconde, ce qui change un peu la phase des
notes (rapport signal / erreur de l'ordre de 15 dB face au rendu exact du moteur) mais pas le son : même spectre, mêmes notes.

## Taille
40 morceaux : 8 Mo en `.vgz` (contre 19 Mo d'OGG). Un morceau FM seul fait 20 à 70 Ko ; ceux qui jouent des échantillons au DAC (batterie,
voix) de 0,2 à 1 Mo.

## Vérifier
```
lsctl evt-render morceau.evt 30 sortie.wav     # les écritures exactes du moteur, par les puces Rust : identique au WAV du moteur
lsctl vgm-render music_01.vgz sortie.wav       # un fichier VGM : intro et deux tours de boucle
```

## Format (rappel)
En-tête de 0x80 octets (`Vgm `, version 1.50, horloge SN76489 3579545, horloge YM2612 7670454, nombre d'échantillons, offset de la
boucle), puis des commandes : `0x52 aa dd` (YM2612 port 0), `0x53 aa dd` (port 1), `0x50 dd` (PSG), `0x61 nnnn` (attendre n échantillons),
`0x66` (fin). La spécification : https://vgmrips.net/wiki/VGM_Specification
