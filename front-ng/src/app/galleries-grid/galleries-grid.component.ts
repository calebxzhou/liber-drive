import { Component, HostListener, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { Gallery, GalleryInfo, MediaItem } from "../media/media";
import { CommonModule } from "@angular/common"; 
import { GalleryItemComponent } from "../gallery-item/gallery-item.component";
import { ActivatedRoute, RouterModule } from "@angular/router";
import { Title } from "@angular/platform-browser";

@Component({
  selector: "gallery-grid",
  templateUrl: "./galleries-grid.component.html",
  imports: [CommonModule, GalleryItemComponent,RouterModule],
  standalone: true,
})
export class GalleriesGridComponent implements OnInit {
  galleryInfo$: GalleryInfo[] = [];

  constructor(private route: ActivatedRoute,private mserv: MediaService) {}
 

  ngOnInit() {
    this.mserv.allGalleriesInfo$.subscribe((g) => {
      this.galleries=this.mserv.processGalleries(g);
    });
    this.mserv.fetchGalleryInfos(); 
  }
  ngOnInit() {
    this.galleryInfo$ = this.route.paramMap.pipe(
      switchMap((params: ParamMap) =>
        this.mserv.getG(params.get('name')!))
    );
  }
}
