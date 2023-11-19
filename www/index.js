/**
 * 文件类型与扩展名
 * @type {{zip: string[], excel: string[], img: string[], code: string[], ppt: string[], imgRaw: string[], text: string[], video: string[], word: string[]}}
 */
const FILE_TYPE_EXTENSION = {
    zip: ['zip','rar','7z','gz'],
    code: ['c','py','rs','java','sh'],
    excel: ['xls','xlsx'],
    img: ['jpg','png','tiff','bmp','gif', 'webp', 'svg'],
    imgRaw: ['cr2','orf','rw2'],
    ppt: ['ppt','pptx'],
    text: ['txt'],
    video: ['mp4', 'mkv', 'flv', 'gif', 'avi', 'mov', 'wmv'],
    word: ['doc','docx'],
}
/**
 * 当前访问路径，每个元素都是目录名
 * @type {string[]}
 */
let PATH= []
/**
 * 当前目录下全部文件
 * @type {File[]}
 */
let FILES = []

/**
 * 文件
 */
class File {
    /**
     * @param name 文件名
     */
    constructor(name) {
        this.name = name;
    }

    /**
     * 获取文件扩展名
     * @returns {string}
     */
    getExtension() {
        return this.name.slice((this.name.lastIndexOf(".") - 1 >>> 0) + 2).toLowerCase();
    }

    /**
     * 获取文件类型
     * @returns {string}
     */
    getType(){
        return Object.entries(FILE_TYPE_EXTENSION).reduce((acc, [key, value]) => {
            return value.includes(this.getExtension()) ? key : acc;
        }, 'file');
    }
    isDir(){
        return this.name.endsWith("/")
    }
    getFileQueryPath(){
        return '_drive/'+getCurrentPath()+ '/'+this.name;
    }
    getIconSrc(){
        let type;
        if(this.isDir())
            type = "dir"
        else
            type = this.getType()
        return "icons/"+type+".svg";
    }
    /*
    +"<br>🕜"+humanReadableTime(this.time)
            +"<br>🗃️"+this.readableSize(); // replace with your text
     */
    getDomElement(){
        let div = document.createElement('div');
        div.id = "file_div_"+this.name
        let img = document.createElement('img');
        img.id="file_icon_"+this.name
        img.src = this.getIconSrc()
        //文件名
        let p = document.createElement('div');
        p.id="file_p_"+this.name
        p.innerHTML = this.name
        div.appendChild(img);
        div.appendChild(p);
        if(this.isDir()){
            div.onclick = () => goNextDir(this.name)
        }else if(this.getType() === 'img'||this.getType()==='video'){
            div.onclick = () => openViewer(FILES,this)
        }
        else  {
            div.onclick = () => window.open(this.getFileQueryPath(),'_blank')
        }
        return div

    }
    queryDetails(){
        //获取文件尺寸
        fetch(this.getFileQueryPath()+"?&size=1")
            .then(response => response.text())
            .then(data => {
                document.getElementById("file_p_"+this.name).innerHTML +="<br>💾"+humanReadableSize(data)
            })
            .catch((error) => {
                console.error('Error:', error);
            });
        //修改时间
        fetch(this.getFileQueryPath()+"?&mod_time=1")
            .then(response => response.text())
            .then(data => {
                document.getElementById("file_p_"+this.name).innerHTML +="<br>🕒"+humanReadableTime(data)
            })
            .catch((error) => {
                console.error('Error:', error);
            });
        //是图片，就读取缩略图
        if(this.getType() === 'img'){
            document.getElementById("file_icon_"+this.name).src =this.getFileQueryPath()+"?&thumbnail=1"
        }
    }
}
window.onload = function() {
    updateDir("/")
}
function getCurrentPath(){
    return PATH.join("/")
}
function updateDir(){
    let path = getCurrentPath()
    changeTitle(path)
    window.stop()
    clearFileGrid()
    FILES = []
    fetch("_drive/"+path)
        .then(response => response.json())
        .then(data => parseDirData(data))
        .catch((error) => {
            console.error('Error:', error);
        });
}

/**
 * 返回上个目录
 */
function goPrevDir(){
    updateDir(PATH.pop())
}

/**
 * 前往下个目录
 * @param {string} dirName
 */
function goNextDir(dirName){
    updateDir(PATH.push(dirName))
}

function changeTitle(titleStr){
    document.getElementById("title").innerText = titleStr
}

function clearFileGrid(){
    document.getElementById("imageGrid").innerHTML=""
}
function parseDirData(json){
    for(let ele of json){
        FILES.push(new File(ele))
    }

    for (let file of FILES) {
        document.getElementById("imageGrid").appendChild(file.getDomElement());
    }
    for (let file of FILES) {
        file.queryDetails()
    }
}

function humanReadableSize(byte) {
    let bytes = Number(byte)
    if(bytes===0)
        return "0B"
    const units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let i = 0;
    while (bytes >= 1024) {
        bytes /= 1024;
        i++;
    }
    return bytes.toFixed(2) + " " + units[i];
}

/**
 * 转换unix时间戳到人类可读时间
 * @param {number} timestamp
 * @returns {string}
 */
function humanReadableTime(timestamp) {
    const date = new Date(timestamp * 1000);

    const year = date.getFullYear();

    // JavaScript months are 0-indexed, so we add 1 to get the correct month number
    const month = ("0" + (date.getMonth() + 1)).slice(-2);

    const day = ("0" + date.getDate()).slice(-2);

    const hours = ("0" + date.getHours()).slice(-2);

    const minutes = ("0" + date.getMinutes()).slice(-2);

    const seconds = ("0" + date.getSeconds()).slice(-2);

    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}
let currentImageIndex = 0;
let imageVideos = [];
function openViewer(allDriveFiles,currFile){
    imageVideos = allDriveFiles
        .filter((file) => file.getType()==='img'|| file.getType()==='video')
    currentImageIndex = imageVideos.findIndex((image)=>image.name===currFile.name)
    changeImage(currFile)
    document.getElementById('viewer').style.display = 'flex';
}

function closeImageViewer() {
    document.getElementById('viewer').style.display = 'none';
}
function clearImageViewer(){
    document.getElementById('viewerImage').src = "";
    document.getElementById('player').src = "";
    document.getElementById('player').style.display = "none";
}
function nextImage() {
    clearImageViewer();
    currentImageIndex = (currentImageIndex + 1) % imageVideos.length;
    changeImage(imageVideos[currentImageIndex])
}

function prevImage() {
    clearImageViewer();
    currentImageIndex = (currentImageIndex - 1 + imageVideos.length) % imageVideos.length;
    changeImage(imageVideos[currentImageIndex])
}
function changeImage(imageDriveFile){
    document.getElementById('image-viewer-text').innerText = imageDriveFile.name
    let v = document.getElementById('player');
    v.style.display = 'none'
    if (imageDriveFile.getType() === 'img'){
        let image = document.getElementById('viewerImage');
        image.src = imageDriveFile.getFileQueryPath();
    }else if(imageDriveFile.getType() === 'video'){
        v.style.display='flex'
        v.src = imageDriveFile.getFileQueryPath();
    }


}
function showExif(){
    let image = document.getElementById('viewerImage');
    EXIF.getData(image, function() {
        alert(EXIF.pretty(this));
    });
}
function downloadImage() {
    const link = document.createElement('a');
    link.href = imageVideos[currentImageIndex].getFileQueryPath();
    link.download = 'image.jpg';
    link.click();
}