import { CommonModule } from '@angular/common';
import { AfterViewInit, Component, ElementRef, HostListener, Input, OnInit, ViewChild } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { MediaService } from '../media/media.service';
import { Media } from '../media/media';
import { toReadableSize } from '../util';
import { ImageTbnlComponent } from '../image-tbnl/image-tbnl.component';
import { MAX_TBNL_SIDELEN, MIN_TBNL_SIDELEN } from '../const';

@Component({
  selector: 'lg-image-grid',
  standalone: true,
  imports: [CommonModule,ImageTbnlComponent],
  templateUrl: './image-grid.component.html',
  styles: ` 
  `
})
export class ImageGridComponent implements OnInit,AfterViewInit{
  @ViewChild('imageGrid') imageGridRef!: ElementRef;
  @Input() medias!: Media[] ;
  galleryName: string="";
  albumName: string="";
  tbnlSidelen=MIN_TBNL_SIDELEN;
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private mediaService: MediaService
  ) {}
  ngAfterViewInit(): void {
    this.calculateSidelen();
  }
  @HostListener('window:resize', ['$event'])
  onResize(_event: any) {
    this.calculateSidelen();
  }
  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      this.galleryName = params.get("galleryName")!;
      this.albumName = params.get("albumName")!; 
    
      
    });
  }
  thumbnailUrl(media:Media){
    return this.mediaService.fetchMediaUrl(this.galleryName,this.albumName,media.name,1);
  }
  isVideo(media:Media){
    return this.mediaService.isVideo(media);
  }
  size(media: Media){
    return toReadableSize(media.size);
  } 
  calculateSidelen(){
    const gridWidth = this.imageGridRef.nativeElement.offsetWidth;
    //一行能放几张
    let imgAmount =  Math.ceil (gridWidth/MIN_TBNL_SIDELEN);
    //剩余空间
    let remain = gridWidth%MIN_TBNL_SIDELEN;
    //分给每个图片的空间
    let each = remain/imgAmount;
    this.tbnlSidelen = MIN_TBNL_SIDELEN+ each

    console.log(imgAmount,remain,each,this.tbnlSidelen) 
  } 
}
