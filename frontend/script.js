var tilesets = {};
var selectedMetatile = null;
var metatiles = {};

function copyTile(originImage, tileX, tileY, destinationImage) {
    let originCanvas = document.createElement('canvas');
    originCanvas.width = originImage.naturalWidth;
    originCanvas.height = originImage.naturalHeight;
    originCanvas.getContext('2d').drawImage(originImage, 0, 0, originImage.naturalWidth, originImage.naturalHeight);
    let originCtx = originCanvas.getContext("2d");

    let destinationCanvas = document.createElement("canvas");
    destinationCanvas.width = 16;
    destinationCanvas.height = 16;
    let destinationCtx = destinationCanvas.getContext("2d");

    let px = tileX * 16;
    let py = tileY * 16;
    for (let y = 0; y < 16; y++) {
        for (let x = 0; x < 16; x++) {
            let pixelData = originCtx.getImageData(px + x, py + y, 1, 1).data;
            destinationCtx.fillStyle = "rgba(" + pixelData[0] + "," + pixelData[1] + "," + pixelData[2] + "," + (pixelData[3] / 255) + ")";
            destinationCtx.fillRect(x, y, 1, 1);
        }
    }
    destinationImage.src = destinationCanvas.toDataURL();
}

function mergeTiles(bottomTile, topTile, destinationImage) {
    let bottomCanvas = document.createElement('canvas');
    bottomCanvas.width = 16;
    bottomCanvas.height = 16;
    bottomCanvas.getContext('2d').drawImage(bottomTile, 0, 0, 16, 16);
    let bottomCtx = bottomCanvas.getContext("2d");

    let topCanvas = document.createElement('canvas');
    topCanvas.width = 16;
    topCanvas.height = 16;
    topCanvas.getContext('2d').drawImage(topTile, 0, 0, 16, 16);
    let topCtx = topCanvas.getContext("2d");

    let destinationCanvas = document.createElement("canvas");
    destinationCanvas.width = 16;
    destinationCanvas.height = 16;
    let destinationCtx = destinationCanvas.getContext("2d");

    for (let y = 0; y < 16; y++) {
        for (let x = 0; x < 16; x++) {
            let topPixel = topCtx.getImageData(x, y, 1, 1).data;
            let bottomPixel = bottomCtx.getImageData(x, y, 1, 1).data;
            let pixelData = topPixel[0] === 0xff && topPixel[1] === 0 && topPixel[2] === 0xfd ? bottomPixel : topPixel;
            destinationCtx.fillStyle = "rgba(" + pixelData[0] + "," + pixelData[1] + "," + pixelData[2] + "," + (pixelData[3] / 255) + ")";
            destinationCtx.fillRect(x, y, 1, 1);
        }
    }
    destinationImage.src = destinationCanvas.toDataURL();
}

function updateSelectedMetatile() {
    let metatile = metatiles[selectedMetatile];
    console.log("updating selected metatile");
    document.getElementById("bottomTile").src = metatile.bottomTile.src;
    document.getElementById("topTile").src = metatile.topTile.src;
    setTimeout(function () {
        mergeTiles(metatile.bottomTile, metatile.topTile, metatile.combinedTile);
        document.getElementById("combinedTile").src = metatile.combinedTile.src;
        metatile.image.src = metatile.combinedTile.src;
    }, 200);

    // add css class to selected image
    for (let id in metatiles) {
        if (Object.prototype.hasOwnProperty.call(metatiles, id)) {
            if (id === selectedMetatile) {
                metatile.image.setAttribute("class", "tile selected");
            } else {
                metatile.image.setAttribute("class", "tile");
            }
        }
    }
}

function download(filename, text) {
    let element = document.createElement('a');
    element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
    element.setAttribute('download', filename);
    element.style.display = 'none';
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
}

function getTilesetFromFileName(fileName) {
    for (let id in tilesets) {
        if (Object.prototype.hasOwnProperty.call(tilesets, id)) {
            let tileset = tilesets[id];
            if (tileset.fileName === fileName) {
                return tileset;
            }
        }
    }
    return null;
}

function outputMetatileDefinitions() {
    let output = "";
    for (let id in tilesets) {
        if (Object.prototype.hasOwnProperty.call(tilesets, id)) {
            let tileset = tilesets[id];
            let fileName = tileset.fileName;
            output += id + "=" + fileName + "\n";
        }
    }

    let i = 0;
    for (let id in metatiles) {
        if (Object.prototype.hasOwnProperty.call(metatiles, id)) {
            let metatile = metatiles[id];
            let topTileData = metatile["topTileData"];
            let bottomTileData = metatile["bottomTileData"];
            console.log("top: " + JSON.stringify(topTileData) + " bottom: " + JSON.stringify(bottomTileData));
            let topImg = topTileData === undefined ? tilesets[Object.keys(tilesets)[0]].id : topTileData.imageId;
            let bottomImg = bottomTileData === undefined ? tilesets[Object.keys(tilesets)[0]].id : bottomTileData.imageId;
            let topIndex = topTileData === undefined ? 0 : topTileData.tileId;
            let bottomIndex = bottomTileData === undefined ? 0 : bottomTileData.tileId;
            output += bottomImg + "," + bottomIndex + "," + topImg + "," + topIndex + (i < 7 ? " " : "");

            // extra new lines
            i++;
            if (i >= 8) {
                output += "\n";
                i = 0;
            }
        }
    }
    return output;
}

window.onload = function () {
    for (let tileName of ["bottomTile", "topTile", "combinedTile"]) {
        let canvas = document.createElement("canvas");
        canvas.width = 16;
        canvas.height = 16;
        let tile = canvas.getContext("2d");
        tile.beginPath();
        tile.rect(0, 0, 16, 16);
        tile.fillStyle = "#ff00fd";
        tile.fill();
        document.getElementById(tileName).src = canvas.toDataURL();
    }

    let metatilesContainer = document.getElementById("metatiles");
    for (let i = 0; i < 16 * 8; i++) {
        let canvas = document.createElement("canvas");
        canvas.width = 16;
        canvas.height = 16;
        let tile = canvas.getContext("2d");
        tile.beginPath();
        tile.rect(0, 0, 16, 16);
        tile.fillStyle = "#ff00fd";
        tile.fill();
        let image = new Image();
        image.src = canvas.toDataURL();
        image.setAttribute("class", "tile");
        image.addEventListener("click", function (event) {
            selectedMetatile = i;
            updateSelectedMetatile();
        });
        let combinedTile = new Image();
        let bottomTile = new Image();
        let topTile = new Image();
        combinedTile.src = canvas.toDataURL();
        bottomTile.src = canvas.toDataURL();
        topTile.src = canvas.toDataURL();
        metatiles[i] = {image: image, combinedTile: combinedTile, bottomTile: bottomTile, topTile: topTile};
        metatilesContainer.appendChild(image);
    }

    document.getElementById("export").addEventListener("click", function (event) {
        let output = outputMetatileDefinitions();
        console.log(output);
        download("metatile_definitions.txt", output);
    });

    //Check File API support
    if (window.File && window.FileList && window.FileReader) {
        document.getElementById("import").addEventListener("change", function (event) {
            let file = event.target.files[0];
            let fileReader = new FileReader();
            fileReader.addEventListener("load", function (progressEvent) {
                let text = progressEvent.target.result;
                console.log(text);
                // Parse metatile definitions
                let lines = text.split("\n");
                let metatileIndex = 0;
                let fileMap = {};
                for (const line of lines) {
                    if (line.trim().length <= 0 || line.startsWith("#")) {
                        continue;
                    }
                    if (line.includes("=")) {
                        // file variable definition
                        let parts = line.split("=");
                        let variable = parts[0];
                        fileMap[variable] = parts[1];
                    } else {
                        // metatile definitions
                        console.log(line);
                        let parts = line.split(" ");
                        for (const part of parts) {
                            let metatileParts = part.split(",");
                            let bottomFile = fileMap[metatileParts[0]];
                            let bottomTileIndex = metatileParts[1];
                            let topFile = fileMap[metatileParts[2]];
                            let topTileIndex = metatileParts[3];

                            let metatile = metatiles[metatileIndex];

                            console.log(fileMap);
                            console.log(parts + " -> " + part + " -> " + metatileParts);
                            console.log(bottomFile + " " + topFile);
                            let bottomTileset = getTilesetFromFileName(bottomFile);
                            let topTileset = getTilesetFromFileName(topFile);

                            let bottomTilesetXMax = Math.floor(bottomTileset.image.naturalWidth / 16);
                            let topTilesetXMax = Math.floor(topTileset.image.naturalWidth / 16);
                            let bottomTileX = bottomTileIndex % bottomTilesetXMax;
                            let topTileX = topTileIndex % topTilesetXMax;
                            let bottomTileY = Math.floor(bottomTileIndex / bottomTilesetXMax);
                            let topTileY = Math.floor(topTileIndex / topTilesetXMax);

                            copyTile(bottomTileset.image, bottomTileX, bottomTileY, metatile["bottomTile"]);
                            copyTile(topTileset.image, topTileX, topTileY, metatile["topTile"]);

                            metatile["bottomTileData"] = {
                                imageId: getTilesetFromFileName(bottomFile).id,
                                x: bottomTileX,
                                y: bottomTileY,
                                tileId: bottomTileIndex
                            };

                            metatile["topTileData"] = {
                                imageId: getTilesetFromFileName(topFile).id,
                                x: topTileX,
                                y: topTileY,
                                tileId: topTileIndex
                            };

                            selectedMetatile = metatileIndex;
                            updateSelectedMetatile();
                            metatileIndex++;
                        }
                    }
                }
            });
            fileReader.readAsText(file);
        });

        var filesInput = document.getElementById("files");

        filesInput.addEventListener("change", function (event) {
            var files = event.target.files; //FileList object
            var output = document.getElementById("uploads");
            output.innerHTML = "";

            for (var i = 0; i < files.length; i++) {
                var file = files[i];

                //Only pics
                if (!file.type.match('image'))
                    continue;
                const fileName = file.name;

                var picReader = new FileReader();
                picReader.addEventListener("load", function (event) {
                    var picFile = event.target;
                    var div = document.createElement("div");
                    div.setAttribute("class", "tilesetContainer");
                    const id = fileName.replace(/[\W_]+/g, "");

                    div.innerHTML = "<span>" + fileName + "</span><img id='" + id + "' class='tileset' src='" + picFile.result + "'" +
                        "title='" + picFile.name + "' alt='failed to preview image'/>";

                    output.insertBefore(div, null);
                    let img = document.getElementById(id);

                    tilesets[id] = {id: id, fileName: fileName, div: div, data: picFile.result, image: img};

                    const clickListener = function (event, leftClick) {
                        if (selectedMetatile == null) {
                            return;
                        }
                        let bounds = img.getBoundingClientRect();
                        let left = bounds.left;
                        let top = bounds.top;
                        let x = event.x - left;
                        let y = event.y - top;
                        let cw = img.clientWidth;
                        let ch = img.clientHeight;
                        let iw = img.naturalWidth;
                        let ih = img.naturalHeight;
                        let px = x / cw * iw;
                        let py = y / ch * ih;
                        let tileX = Math.floor(px / 16);
                        let tileY = Math.floor(py / 16);
                        let tileId = Math.floor(img.naturalWidth / 16) * tileY + tileX;
                        console.log("clicked (" + tileX + ", " + tileY + ") #" + tileId);
                        let selectedMetatilePartName = leftClick ? "bottomTile" : "topTile";
                        let selectedMetatilePart = document.getElementById(selectedMetatilePartName);
                        copyTile(img, tileX, tileY, selectedMetatilePart);
                        metatiles[selectedMetatile][selectedMetatilePartName].src = selectedMetatilePart.src;
                        metatiles[selectedMetatile][selectedMetatilePartName + "Data"] = {
                            imageId: id,
                            x: tileX,
                            y: tileY,
                            tileId: tileId
                        };
                        updateSelectedMetatile();
                    };

                    img.addEventListener('contextmenu', function (event) {
                        event.preventDefault();
                        clickListener(event, false);
                        return false;
                    }, false);
                    img.addEventListener('click', function (event) {
                        clickListener(event, true);
                    });
                });

                //Read the image
                picReader.readAsDataURL(file);
            }

        });
    } else {
        document.getElementById("uploads").innerHTML = "ERROR: Your browser does not support File API!";
    }
};