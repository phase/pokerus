# Notes

save metatiles from porymap:
```c++
void Project::saveTilesetMetatiles(Tileset *tileset) {
    QFile metatiles_file(tileset->metatiles_path);
    if (metatiles_file.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
        QByteArray data;
        for (Metatile *metatile : *tileset->metatiles) {
            for (int i = 0; i < 8; i++) {
                Tile tile = metatile->tiles->at(i);
                uint16_t value = static_cast<uint16_t>((tile.tile & 0x3ff)
                                                    | ((tile.xflip & 1) << 10)
                                                    | ((tile.yflip & 1) << 11)
                                                    | ((tile.palette & 0xf) << 12));
                data.append(static_cast<char>(value & 0xff));
                data.append(static_cast<char>((value >> 8) & 0xff));
            }
        }
        metatiles_file.write(data);
    } else {
        tileset->metatiles = new QList<Metatile*>;
        logError(QString("Could not open tileset metatiles file '%1'").arg(tileset->metatiles_path));
    }
}
```
```
tile id = 0000 0011 1111 1111    (& 0x3ff)
xflip   = 0000 0100 0000 0000    (<< 10)
yflip   = 0000 1000 0000 0000    (<< 11)
palette = 1111 0000 0000 0000    ((p & 0xf) << 12)
                   \----|----/
         \----|----/    first byte (value & 0xff)
              second byte (value & 0xff)
```