// https://sbapi-team.github.io/SmileBASIC-FileParser/
// i don't know how to do this

// hacked together from https://12me21.github.io/sbtools/
// to https://deno.land or whatev er

// updated to trim a buncha unused stuff
// this is an artifact, really -- made it to convert txt to prg and nothing else
// could try to get the rest working but i'm a fool

// i have no concept of current directory
/** @type {string} */
const FILENAME = Deno.args[0] || "./prg.sb";
/** @type {string} */ // FIXME: ew
const OUTNAME = Deno.args[1] || ("./T" + FILENAME.substring(2).split(".")[0].toUpperCase());
/** @type {string} */
const TEXT = await Deno.readTextFile(FILENAME);

var fileData = new Uint8Array(0);

// "why is this like this"
// because i'm too lazy to rip out the gui from the conversion code.
// and i dont' know enough.
var $fileType = 0;
var $compression = false;
var $dataType = null;
var $dimensions = 1;
var $dimension1 = 0, $dimension2 = 0, $dimension3 = 0, $dimension4 = 0;
var $icon = 1;
var $date = [0, 2021, 5, 31], $time = [0, 13, 12], $seconds= 1;
var $author = "V360Tech", $uid = 42069; // very cool name you got there

function main() {
	let bytes = new Uint8Array(fuckinUTF8Bytes(TEXT));
	let file = writeFile(readSettings(), bytes);
	// if (typeof file !== "Uint8Array") return console.warn("fuckin");
	Deno.writeFile(OUTNAME, file);
}

function fuckinUTF8Bytes(str) {
	var utf8 = unescape(encodeURIComponent(str));
	var arr = [];
	for (var i = 0; i < utf8.length; i++) {
		arr.push(utf8.charCodeAt(i));
	}
	return arr;
}

// "util"
function col16(a, r, g, b) {
	return (a == 255 ? 1 : 0) | b >> 3 << 1 | g >> 3 << 5 + 1 | r >> 3 << 5 + 5 + 1;
}
function clamp(value, min, max) {
	return Math.min(Math.max(value, min), max);
}
function pad(number, length) {
	return number.toString().padStart(length, "0");
}

// external shit
// sha1hmac.js
function sha1_hmac(msg, key) {
	if (key.length > 64)// keys longer than blocksize are shortened
		key = sha1(key);
	var oKeyPad = new Uint8Array(64 + 20);
	var iKeyPad = new Uint8Array(64 + msg.length);
	for (var i = 0; i < 64; ++i) {
		oKeyPad[i] = key[i] ^ 0x5C;
		iKeyPad[i] = key[i] ^ 0x36;
	}
	iKeyPad.set(msg, 64);
	var iPadRes = sha1(iKeyPad);
	oKeyPad.set(iPadRes, 64)

	return sha1(oKeyPad);
};
function sha1(msg) {
	function rotate_left(n, s) {
		var t4 = (n << s) | (n >>> (32 - s));
		return t4;
	}
	var blockstart; var i, j;
	var W = new Array(80);
	var H0 = 0x67452301; var H1 = 0xEFCDAB89;
	var H2 = 0x98BADCFE; var H3 = 0x10325476;
	var H4 = 0xC3D2E1F0;
	var A, B, C, D, E, temp;
	var msg_len = msg.length;
	var word_array = [];
	for (i = 0; i < msg_len - 3; i += 4) {
		j = msg[i] << 24 | msg[i + 1] << 16 | msg[i + 2] << 8 | msg[i + 3];
		word_array.push(j);
	}
	switch (msg_len % 4) {
		case 0: i = 0x080000000; break;
		case 1: i = msg[msg_len - 1] << 24 | 0x0800000; break;
		case 2: i = msg[msg_len - 2] << 24 | msg[msg_len - 1] << 16 | 0x08000; break;
		case 3: i = msg[msg_len - 3] << 24 | msg[msg_len - 2] << 16 | msg[msg_len - 1] << 8 | 0x80; break;
	}
	word_array.push(i);
	while ((word_array.length % 16) != 14) word_array.push(0);
	word_array.push(msg_len >>> 29);
	word_array.push((msg_len << 3) & 0x0ffffffff);
	for (blockstart = 0; blockstart < word_array.length; blockstart += 16) {
		for (i = 0; i < 16; i++)
			W[i] = word_array[blockstart + i];
		for (i = 16; i <= 79; i++)
			W[i] = rotate_left(W[i - 3] ^ W[i - 8] ^ W[i - 14] ^ W[i - 16], 1);
		A = H0; B = H1; C = H2; D = H3; E = H4;
		for (i = 0; i <= 19; i++) {
			temp = (rotate_left(A, 5) + ((B & C) | (~B & D)) + E + W[i] + 0x5A827999) & 0xffffffff;
			E = D; D = C; C = rotate_left(B, 30); B = A; A = temp;
		}
		for (i = 20; i <= 39; i++) {
			temp = (rotate_left(A, 5) + (B ^ C ^ D) + E + W[i] + 0x6ED9EBA1) & 0xffffffff;
			E = D; D = C; C = rotate_left(B, 30); B = A; A = temp;
		}
		for (i = 40; i <= 59; i++) {
			temp = (rotate_left(A, 5) + ((B & C) | (B & D) | (C & D)) + E + W[i] + 0x8F1BBCDC) & 0xffffffff;
			E = D; D = C; C = rotate_left(B, 30); B = A; A = temp;
		}
		for (i = 60; i <= 79; i++) {
			temp = (rotate_left(A, 5) + (B ^ C ^ D) + E + W[i] + 0xCA62C1D6) & 0xffffffff;
			E = D; D = C; C = rotate_left(B, 30); B = A; A = temp;
		}
		H0 = (H0 + A) & 0xffffffff; H1 = (H1 + B) & 0xffffffff;
		H2 = (H2 + C) & 0xffffffff; H3 = (H3 + D) & 0xffffffff;
		H4 = (H4 + E) & 0xffffffff;
	}
	var result = new Uint8Array(20);
	var dataview = new DataView(result.buffer);
	dataview.setUint32(0, H0, false);
	dataview.setUint32(4, H1, false);
	dataview.setUint32(8, H2, false);
	dataview.setUint32(12, H3, false);
	dataview.setUint32(16, H4, false);
	return result;
};

// external shit
// sbfile
////////////////////////
//// Headers Format ////
////////////////////////
const HEADER = [
	{ pos: 0x00, type: "Int16", value: 0x0001 }, // 0 = builtin, 1 = saved by user?
	{ pos: 0x02, type: "Int16", name: "fileType" }, // 0 = TXT, 1 = DAT
	{ pos: 0x04, type: "Int16", name: "compression" }, // 1 = zlib compressed
	{ pos: 0x06, type: "Int16", name: "icon" }, // 0 = TXT/DAT, 1 = PRG/GRP (depending on fileType)
	{ pos: 0x08, type: "Int32", name: "fileSize" }, // Size of data, not including header/footer (display only)
	{ pos: 0x0C, type: "Int16", name: "year" }, // 
	{ pos: 0x0E, type: "Int8", name: "month" }, // 
	{ pos: 0x0F, type: "Int8", name: "day" }, // Modified date/time
	{ pos: 0x10, type: "Int8", name: "hour" }, // 
	{ pos: 0x11, type: "Int8", name: "minute" }, // 
	{ pos: 0x12, type: "Int8", name: "second" }, // 
	//{pos:0x13, type:"Int8", value:0x3}, Unknown
	{ pos: 0x14, type: "String8", arg: 18, name: "author1" }, //hidden
	{ pos: 0x26, type: "String8", arg: 18, name: "author2" }, //displayed, but replaced with author1 when uploaded
	{ pos: 0x38, type: "Int32", name: "uid1" }, //whatever
	{ pos: 0x3C, type: "Int32", name: "uid2" },
	//{pos: 0x40, type: "Int16", name: "unused"},
], HEADER_SIZE = 0x50;
const DAT_HEADER = [
	{ pos: 0x00, type: "Uint32", value: 0x4E424350 }, //PCBN
	{ pos: 0x04, type: "Uint32", value: 0x31303030 }, //0001
	{ pos: 0x08, type: "Int16", name: "dataType" }, //3 = uint16 (colors), 4 = int32, 5 = double
	{ pos: 0x0A, type: "Int16", name: "dimensions" },
	{ pos: 0x0C, type: "Int32", name: "dimension1" },
	{ pos: 0x10, type: "Int32", name: "dimension2" },
	{ pos: 0x14, type: "Int32", name: "dimension3" },
	{ pos: 0x18, type: "Int32", name: "dimension4" },
], DAT_HEADER_SIZE = 28;
const FOOTER_SIZE = 20;
//////////
////  ////
//////////
//header: Object
//  data: Uint8Array
//return: Uint8Array
function writeFile(header, data) {
	const IS_DAT = false; //header.fileType == 1;
	var dataLength = data.length;
	console.log(`data length: ${dataLength}`);
	
	//pad data length
	if (IS_DAT) {
		console.warn("entered an invalid zone. i don't like it");
		return "oops";
	}

	if (header.compression) {
		console.warn("entered an invalid zone. i don't like it");
		return "oops";
	}

	var file = new Uint8Array(HEADER_SIZE + dataLength + FOOTER_SIZE);
	header.fileSize = dataLength;

	file.set(data, HEADER_SIZE);

	templateSet(file, HEADER, header, 0);

	setFooter(file);
	return file;
}
//  file: Uint8Array
//header: Object
//return: Uint8Array
function readFile(file, header) {
	templateGet(file, HEADER, header, 0);
	var isDat = header.fileType == 1;
	var data = file.slice(HEADER_SIZE, file.length - FOOTER_SIZE);
	if (header.compression) {
		data = pako.inflate(data);
	}
	if (isDat) {
		templateGet(data, DAT_HEADER, header, 0);
		data = data.slice(DAT_HEADER_SIZE);
	}
	return data;
}
////////////////
//// Footer ////
////////////////
var HMAC_KEY = [
	110,113,109, 98,121, 43,101, 57
	,83, 63,123, 37, 85, 42, 45, 86
	,93, 53, 49,110, 37, 94,120, 90
	,77,107, 56, 62, 98,123, 63,120
	,93, 38, 63, 40, 78,109,109, 86
	,91, 44,103, 56, 53, 58, 37, 54
	,83,113,100, 34, 39, 85, 34, 41
	,47, 56,117, 55, 55, 85, 76, 50
];
//file: Uint8Array
function setFooter(file) {
	var withoutFooter = file.length - FOOTER_SIZE;
	file.set(sha1_hmac(new Uint8Array(file.buffer, 0, withoutFooter), HMAC_KEY), withoutFooter);
}
///////////////////
//// Templates ////
///////////////////
//    file: Uint8Array
//template: Array
//  header: Object
//  offset: Number
function templateSet(file, template, header, offset) {
	var view = new DataView(file.buffer);
	template.forEach(function(item) {
		var value = item.name ? header[item.name] : item.value;
		var arg = item.arg === undefined ? true : item.arg;
		view["set" + item.type](item.pos + +offset, value, arg);
	});
}
//    file: Uint8Array
//template: Array
//  header: Object
//  offset: Number
function templateGet(file, template, header, offset) {
	var view = new DataView(file.buffer);
	template.forEach(function(item) {
		const ARG = item.arg === undefined ? true : item.arg;
		var value = view["get" + item.type](item.pos + +offset, ARG);
		if (item.name) {
			header[item.name] = value;
		} else if (value != item.value) {
			console.warn("template value error");
			//return null;
		}
	});
}
////////////////////////
//// DataView UTF-8 ////
////////////////////////
DataView.prototype.getString8 = function(pos, length) {
	var string = "";
	for (var i = 0; i < length; i++) {
		var chr = this.getUint8(pos + i, true);
		if (chr == 0) break;
		string += String.fromCharCode(chr);
	}
	return string;
}
DataView.prototype.setString8 = function(pos, string, length) {
	string = string.substr(0, length - 1);
	for (var i = 0; i < string.length; i++)
		this.setUint8(pos + i, string.charCodeAt(i));
	this.setUint8(pos + i, 0);
}
//credits:
//record, Y, snail, trinitro, 12Me21




// time
function setCurrentTime() {
	var now = new Date();
	writeDateTime(now.getFullYear(), now.getMonth() + 1, now.getDate(), now.getHours(), now.getMinutes());
	$seconds = now.getSeconds();
}
function writeDateTime(year, month, day, hour, minute) {
	$date = pad(clamp(year, 0, 9999), 4) + "-" + pad(clamp(month, 0, 99), 2) + "-" + pad(clamp(day, 0, 99), 2);
	$time = pad(clamp(hour, 0, 99), 2) + ":" + pad(clamp(minute, 0, 99), 2);
}

function fileUpload(filename) {
	var reader = new FileReader();
	reader.onload = function(z) {
		fileData = new Uint8Array(reader.result);
		$dataType = 4;
		$dimensions = 1;
		$dimension1 = Math.ceil(fileData.length / 4);
	};
	reader.readAsArrayBuffer(filename);
}

function imageUpload(x) {
	var reader = new FileReader();
	reader.onload = function(z) {
		var i = new Image;
		i.onload = ()=>imageToGrp(i);
		i.src = reader.result;
	}
	reader.readAsDataURL(x.files[0]);
}


function finish() {
	var header = readSettings();
	const prefix = header.fileType == 1 ? "B" : "T";
	var file = writeFile(readSettings(), fileData);
	doSave(file, prefix + $name);
}
function doSave(data, name) {
	var blob = new Blob([data], { "type": "octet/stream" });
	// FIXME: this will save differenteley
	// $download.download=name;
	// $download.href=URL.createObjectURL(blob);
	// $download.click();
}

function imageToGrp(image) {
	console.warn("entered an invalid zone. i don't like it");
	return "oops";
}

// make a header from the settings
function readSettings() {
	var header = {};
	header.fileType = +$fileType;
	const IS_DAT = header.fileType == 1;
	header.icon = +$icon;
	// var date = readDateTime();
	// header.year = date[0];
	// header.month = date[1];
	// header.day = date[2];
	// header.hour = date[3];
	// header.minute = date[4];
	header.second = +$seconds;
	header.author1 = header.author2 = $author;
	header.uid1 = header.uid2 = +$uid;
	header.compression = +$compression;
	if (IS_DAT) {
		header.dataType = +$dataType;
		header.dimensions = +$dimensions;
		header.dimension1 = +$dimension1;
		header.dimension2 = +$dimension2;
		header.dimension3 = +$dimension3;
		header.dimension4 = +$dimension4;
	}
	return header;
}

main();
