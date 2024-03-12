import { CommonModule } from '@angular/common';
import { Component, Input, OnInit } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { MediaService } from '../media/media.service';
import { Media } from '../media/media';
import { toReadableSize } from '../util';

@Component({
  selector: 'lg-image-grid',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './image-grid.component.html',
  styles: `
  .img-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(128px, 1fr));
    grid-auto-flow: dense;
    grid-gap: 0.5px;
  }
  `
})
export class ImageGridComponent implements OnInit{

  @Input() medias!: Media[] 
  galleryName: string="";
  albumName: string="";
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private mediaService: MediaService
  ) {}
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
  groupMediaByDay(arg0: Media[]): Record<string, Media[]> {
    return this.mediaService.groupMediaByDay(arg0)
  }
}
