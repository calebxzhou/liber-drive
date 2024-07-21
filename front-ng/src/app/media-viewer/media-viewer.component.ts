import { CommonModule, PlatformLocation } from "@angular/common";
import {
  AfterViewInit,
  CUSTOM_ELEMENTS_SCHEMA,
  Component,
  ElementRef,
  EventEmitter,
  HostListener,
  Input,
  OnDestroy,
  OnInit,
  Output,
  QueryList,
  Renderer2,
  ViewChild,
  ViewChildren,
} from "@angular/core";
import { Media, ImageExif } from "../media/media";
import { mapTools } from "../gcj";
import { MediaService } from "../media/media.service";
import { readableDateTime, toReadableSize } from "../util";
import { Location } from "@angular/common";
import { Router, ActivatedRoute } from "@angular/router";
import { LBS_KEY, LOADING_GIF } from "../const";
import { MatSnackBar } from "@angular/material/snack-bar";
import {
  HttpClient,
  HttpClientJsonpModule,
  HttpClientModule,
  HttpEvent,
  HttpEventType,
} from "@angular/common/http";
// import Swiper styles
import "swiper/css";
import "swiper/css/navigation";
import "swiper/css/pagination";
import { Swiper } from "swiper";
import { FileSaverModule } from "ngx-filesaver";
import { TextSwitcherComponent } from "../text-switcher/text-switcher.component";
import { LazyLoadImageModule } from "ng-lazyload-image";
import { ExifDisplayComponent } from "../exif-display/exif-display.component";
import { GpsService } from "../gps.service";
declare global {
  interface Window {
    locationData: any;
  }
}
declare var TMap: any;
@Component({
  selector: "lg-media-viewer",
  standalone: true,
  imports: [
    CommonModule,
    LazyLoadImageModule,
    FileSaverModule,
    TextSwitcherComponent,
    ExifDisplayComponent,
    HttpClientModule,
    HttpClientJsonpModule,
  ],
  templateUrl: "./media-viewer.component.html",
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
})
export class MediaViewerComponent implements OnInit, OnDestroy, AfterViewInit {
  //所有图片
  @Input() medias!: Media[];
  @Input() index: number = -1;
  @Input() isDisplayViewer!: boolean;
  @Input() date!: string;
  @Input() albumName!: string;
  @Output() isDisplayViewerChange = new EventEmitter<boolean>();
  @Output() onClose = new EventEmitter();
  @ViewChildren("videoPlayer") videoPlayers: QueryList<ElementRef> =
    new QueryList();
  swiper!: Swiper;
  loadProgress = 0;
  //尺寸（载入原图用）
  fullImageSize = "...";
  //地址
  locaStr: string | undefined;
  //距离
  distance: string | undefined;
  //当前图片经纬度
  lat: number | undefined;
  lng: number | undefined;
  //显示图片
  displayingUrl = LOADING_GIF;
  //是否原图
  isOriginalLoaded = false;
  //是否载入视频
  playingVideo: Media | null = null;
  //视频比特率（每秒消耗流量）
  bitrate = "";
  //当前图片缩放比例
  scaleRatio = 1;
  displayMap = false;
  title = "载入中";
  exif: ImageExif | undefined = undefined;
  LOADING_GIF = LOADING_GIF;

  @ViewChild("tmap", { static: false }) tmap!: ElementRef;

  constructor(
    private renderer: Renderer2,
    private router: Router,
    private location: Location,
    private platformLocation: PlatformLocation,
    private route: ActivatedRoute,
    private mediaService: MediaService,
    private snackBar: MatSnackBar,
    private gpsl: GpsService,
    private http: HttpClient
  ) {
    //取消默认返回键逻辑
    history.pushState(null, "", window.location.href); // Prevent the default back action
    // history.pushState(null, "", window.location.href); // Prevent the default back action
  }
  ngAfterViewInit() {
    let params = {
      // Optional parameters
      direction: "horizontal",
      loop: false,
      keyboard: {
        enabled: true,
        onlyInViewport: false,
      },
      // If we need pagination
      pagination: {
        el: ".swiper-pagination",
      },
      speed: 400,
      spaceBetween: 0,
      // Navigation arrows
      navigation: {
        nextEl: ".swiper-button-next",
        prevEl: ".swiper-button-prev",
      },
      initialSlide: this.index,
      // And if we need scrollbar
      scrollbar: {
        el: ".swiper-scrollbar",
      },
      autoHeight: true,
      virtual: {
        enabled: true,
      },
      zoom: {
        maxRatio: 3,
      },
      on: {
        zoomChange: (swiper: Swiper, scale: number) => {
          //放得太大 不允许切图
          swiper.allowTouchMove = scale < 2;
          this.scaleRatio = scale;
        },
      },
    };

    let el = document.querySelector("swiper-container")!;
    Object.assign(el, params);
    el.initialize();
    this.onSwiperIndexChange(el.swiper, this.medias);
    this.swiper = el.swiper;
    el.swiper.on("activeIndexChange", (s: Swiper) =>
      this.onSwiperIndexChange(s, this.medias)
    );
    this.tmap.nativeElement.style.display = "none";
  }
  playVideo(media: Media) {
    this.playingVideo = media;
  }
  scaleOut() {
    this.swiper.zoom.out();
  }
  //改变图片时
  onSwiperIndexChange(swiper: Swiper, medias: Media[]) {
    this.index = swiper.activeIndex;
    let media = medias[this.index];
    //原图大小
    this.fullImageSize = toReadableSize(media.size);

    this.isOriginalLoaded = false;
    this.playingVideo = null;
    if (media.exif) {
      this.title = readableDateTime(media.exif?.shot_time);
      this.exif = media.exif;

      //距离
      let lat = this.gpsl.parseDegMinSec(media.exif.loca.lat);
      let lng = this.gpsl.parseDegMinSec(media.exif.loca.lng);
      if (!isNaN(lat) && !isNaN(lng)) {
        const clientGps = window.locationData;
        let gcj02_latlng = mapTools.transformWGS2GCJ({ lat, lng });
        if (gcj02_latlng) {
          lat = gcj02_latlng.lat;
          lng = gcj02_latlng.lng;
        }
        this.lat = lat;
        this.lng = lng;
        this.distance = this.gpsl
          .calculateDistance(lat, lng, clientGps.lat, clientGps.lng)
          .toFixed(2);
        this.http
          .jsonp<any>(
            `https://apis.map.qq.com/ws/geocoder/v1/?location=${lat},${lng}&key=${LBS_KEY}&get_poi=1&output=jsonp&poi_options=policy=5`,
            "callback"
          )
          .subscribe((loca: any) => {
            console.log(loca);
            let province = loca.result.ad_info.province.substring(0, 2);
            let city = loca.result.ad_info.city.replaceAll("市", "");
            let dist = loca.result.ad_info.district;
            let detailed = loca.result.formatted_addresses.recommend;
            this.locaStr = `${province}${city}${dist}-${detailed}`;
          });
      } else {
        this.lat = undefined;
        this.lng = undefined;
      }
    }
    //暂停视频
    this.videoPlayers.forEach((player) => {
      const videoEl: HTMLVideoElement = player.nativeElement;
      videoEl.pause();
    });
    if (this.isVideo(media) && media.duration) {
      this.bitrate = toReadableSize(media.size / media.duration);
    }
  }
  ngOnInit(): void {
    //禁止滚动条
    this.renderer.setStyle(document.body, "overflow", "hidden");

    //返回键逻辑=关闭
    this.platformLocation.onPopState(() => {
      this.close();
    });
  }
  ngOnDestroy() {
    this.renderer.removeStyle(document.body, "overflow");
  }
  @HostListener("document:keydown.escape", ["$event"])
  onKeydownHandler(event: KeyboardEvent) {
    this.close();
  }

  close() {
    this.isDisplayViewerChange.emit(false);
  }
  loadOriginal(media: Media) {
    let t1 = Date.now();
    this.mediaService
      .fetchMedia(this.albumName, media.name, -1)
      .subscribe((event: HttpEvent<any>) => {
        if (event.type === HttpEventType.DownloadProgress) {
          this.fullImageSize =
            Math.round((100 * event.loaded) / (event.total ?? 1)) + "%";
        } else if (event.type === HttpEventType.Response) {
          let t2 = Date.now();
          const blob: Blob = event.body;
          let imageEl: HTMLImageElement = document.getElementById(
            `img_${media.size}`
          ) as HTMLImageElement;
          // Create a new FileReader instance
          let reader = new FileReader();

          // Define the onload event
          reader.onloadend = function () {
            // Once the read operation is finished, set the image source to the Base64 string
            imageEl.src = reader.result as string;
          };

          // Start reading the blob as a data URL
          reader.readAsDataURL(blob);
          this.isOriginalLoaded = true;
          this.snackBar.open(
            `已载入原图(⏰${((t2 - t1) / 1000).toFixed(2)}s)`,
            "x",
            {
              duration: 1000,
            }
          );
        }
      });
  }
  isImage(media: Media) {
    return this.mediaService.isImage(media);
  }
  isVideo(media: Media) {
    return this.mediaService.isVideo(media);
  }
  mediaUrl(media: Media, full: boolean) {
    return this.mediaService.fetchMediaUrl(
      this.albumName,
      media.name,
      full ? -1 : 0
    );
  }
  toggleMap() {
    if (!this.displayMap) {
      this.tmap.nativeElement.style.display = "block";
      this.displayMap = true;
      var center = new TMap.LatLng(this.lat, this.lng);
      //定义map变量，调用 TMap.Map() 构造函数创建地图
      let map = new TMap.Map(this.tmap.nativeElement, {
        center, //设置地图中心点坐标
        zoom: 10, //设置地图缩放级别
        pitch: 43.5, //设置俯仰角
      });
      var markerLayer = new TMap.MultiMarker({
        map: map, //指定地图容器
        //样式定义
        styles: {
          //创建一个styleId为"myStyle"的样式（styles的子属性名即为styleId）
          myStyle: new TMap.MarkerStyle({
            width: 48, // 点标记样式宽度（像素）
            height: 48, // 点标记样式高度（像素）
            src: "assets/icon/location.svg", //图片路径
            //焦点在图片中的像素位置，一般大头针类似形式的图片以针尖位置做为焦点，圆形点以圆心位置为焦点
            anchor: { x: 16, y: 32 },
          }),
        },
        //点标记数据数组
        geometries: [
          {
            id: "1", //点标记唯一标识，后续如果有删除、修改位置等操作，都需要此id
            styleId: "myStyle", //指定样式id
            position: center, //点标记坐标位置
            properties: {
              //自定义属性
              title: "marker1",
            },
          },
        ],
      });
    } else {
      this.displayMap = false;
      this.tmap.nativeElement.innerHTML = "";
      this.tmap.nativeElement.style.display = "none";
    }
  }
}
