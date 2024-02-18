
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
 *
 * @type {HTMLDivElement}
 */
let IMAGE_ROOT_DIV
/**
 * 当前目录下全部视频
 * @type {HTMLImageElement[]}
 */
let IMAGE_ELEMENTS = []
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
        div.appendChild(img);
        //if(this.getType() !== 'img'){
            //文件名
            let p = document.createElement('span');
            p.id="file_p_"+this.name
            p.innerHTML = this.name
            div.appendChild(p);
        //}
        if(this.isDir()){
            div.onclick = () => goNextDir(this.name)
        }else if(this.getType() === 'img'){
            div.onclick = () => openImageViewer(this)
        }else if(this.getType()==='video'){
            div.onclick = () => openViewer(FILES,this)
        }
        else  {
            div.onclick = () => window.open(this.getFileQueryPath(),'_blank')
        }
        return div

    }
    queryDetails(){
        //读取缩略图
        if(this.getType() === 'img'){
            let icon = document.getElementById("file_icon_"+this.name);
            icon.alt = "【图片："+this.name+"】"
            icon.src =this.getFileQueryPath()+"?&thumbnail=1"
            icon.style.height="128px"
        }
    }
    queryByteSize(){
        //获取文件尺寸
        fetch(this.getFileQueryPath()+"?&size=1")
            .then(response => response.text())
            .then(data => {
                this.byteSize = data
                //document.getElementById("file_p_"+this.name).innerHTML +="<br>💾"+humanReadableSize(data)
            })
            .catch((error) => {
                console.error('Error:', error);
            });
    }
    queryModTime(){
        //修改时间
        fetch(this.getFileQueryPath()+"?&mod_time=1")
            .then(response => response.text())
            .then(data => {
                this.modTime = data
                //document.getElementById("file_p_"+this.name).innerHTML +="<br>🕒"+humanReadableTime(data)
            })
            .catch((error) => {
                console.error('Error:', error);
            });
    }
}
window.onload = function() {
    updateDir("/")
}
/*document.onkeydown = function(event) {
    switch (event.keyCode) {
        case 37: // Left arrow key
            prevImage();
            break;
        case 39: // Right arrow key
            nextImage();
            break;
    }
};*/

function getFileByteSize(fileQueryPath, doOnSuccess){
    //获取文件尺寸
    fetch(fileQueryPath+"?&size=1")

        .then(response => response.text())
        .then(data => {
            doOnSuccess(data)
        })
        .catch((error) => {
            console.error('Error:', error);
        });
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
    IMAGE_ELEMENTS = []
    IMAGE_ROOT_DIV = document.createElement('div')
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
    document.getElementById("fileGrid").innerHTML=""
}

/**
 * @type {Viewer}
 */
let gallery;
function parseDirData(json){
    for(let ele of json){
        FILES.push(new File(ele))
    }

    for (let file of FILES) {
        document.getElementById("fileGrid").appendChild(file.getDomElement());
        if (file.getType() === 'img'){
            let image = document.createElement('img')
            image.src =  file.getFileQueryPath()+"?webp=1"
            image.setAttribute("img_name",file.name)
            IMAGE_ELEMENTS.push(image)
        }
    }
    for (let imgE of IMAGE_ELEMENTS) {
        IMAGE_ROOT_DIV.appendChild(imgE)
    }
    for (let file of FILES) {
        file.queryDetails()
    }
    gallery = new Viewer(IMAGE_ROOT_DIV);
}



/**
 *
 * @param {File[]} allDriveFiles
 * @param {File} currFile
 */
function openImageViewer(currFile){
    /*let div = document.createElement('div');
    let imageFirst = document.createElement('img')
    imageFirst.src =  currFile.getFileQueryPath()+"?webp=1"
    div.appendChild(imageFirst)
    for (let file of allDriveFiles) {
        if(file.getType() === 'img'){
            let image = document.createElement('img')
            image.src =  file.getFileQueryPath()+"?webp=1"
            div.appendChild(image)
        }
    }*/
    let index = IMAGE_ELEMENTS.findIndex((ele) => ele.getAttribute("img_name")==currFile.name)
    gallery.show()
    gallery.view(index)
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

function closeViewer() {
    clearImageViewer();
    document.getElementById('viewer').style.display = 'none';
}
function clearImageViewer(){
    document.getElementById('viewerImage').src = "";
    document.getElementById('player').src = "";
    document.getElementById('player').style.display = "none";
    document.getElementById('show_full_image_btn').style.display = "none";
}
function nextImage() {
    currentImageIndex = (currentImageIndex + 1) % imageVideos.length;
    changeImage(imageVideos[currentImageIndex])
}

function prevImage() {
    currentImageIndex = (currentImageIndex - 1 + imageVideos.length) % imageVideos.length;
    changeImage(imageVideos[currentImageIndex])
}
const SHOW_FULL_IMAGE_STR = '💾查看原图'
function changeImage(imageDriveFile){
    clearImageViewer()
    document.getElementById('image-viewer-text').innerText = imageDriveFile.name
    if (imageDriveFile.getType() === 'img'){
        let image = document.getElementById('viewerImage');
        image.src = imageDriveFile.getFileQueryPath()+"?webp=1";
        let sizeStr = document.getElementById("show_full_image_btn").innerText;
        getFileByteSize(imageDriveFile.getFileQueryPath(),(data)=>
            document.getElementById("show_full_image_btn").innerText =SHOW_FULL_IMAGE_STR  + humanReadableSize(data)
        )
        document.getElementById("show_full_image_btn").style.display = "";
    }else if(imageDriveFile.getType() === 'video'){
        let v = document.getElementById('player');
        v.style.display='block'
        v.src = imageDriveFile.getFileQueryPath();
    }
}
function showFullImage(){
    let image = document.getElementById('current-viewing-img');
    image.src = image.getAttribute("full-picture-url")
    document.getElementById("show-full-image-span").style.display='none'
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