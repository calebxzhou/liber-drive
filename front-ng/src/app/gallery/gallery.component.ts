import { Component, OnInit } from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { Gallery, MediaItem } from "../media/media";
import { Title } from "@angular/platform-browser";
import { CommonModule } from "@angular/common";
import { MediaService } from "../media/media.service";
import { MatIconModule } from "@angular/material/icon";
import { MatButtonModule } from "@angular/material/button";
import { MatToolbarModule } from "@angular/material/toolbar";
import { MediaViewerComponent } from "../media-viewer/media-viewer.component";
@Component({
  selector: "lg-gallery",
  standalone: true,
  imports: [
    CommonModule,
    MatToolbarModule,
    MatIconModule,
    MatButtonModule,
    MediaViewerComponent,
  ],
  templateUrl: "./gallery.component.html",
  styles: ``,
})
export class GalleryComponent implements OnInit {


  title: string = "";
  gallery!: Gallery; // the gallery object to display
  index = 0;
  medias!: MediaItem[];
  displayViewer = false;

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private titleService: Title,
    private mediaService: MediaService
  ) {}

  ngOnInit() {
    // get the id from the route parameter
    this.route.paramMap.subscribe((params) => {
      let name = params.get("name") ?? "";
      this.title = name;

      const galleries = this.mediaService.galleries;
      let gallery = galleries.find((g) => g.name === name);
      if (!gallery) {
        console.error("gallery !exists");
        return;
      }
      this.gallery = gallery;
      this.titleService.setTitle(gallery.name);
    });
  }
  isVideo(media: MediaItem): boolean {
    return this.mediaService.isVideo(media);
  }
  openViewer(index: number, media: MediaItem) {
    this.displayViewer = true;
    this.medias = this.gallery.medias;
    this.index = index;
  }
  // get the unique dates of the medias in yyyyMMdd format
  getDays(medias: MediaItem[]): string[] {
    const days = new Set<string>();
    for (const media of medias) {
      const date = new Date(media.time * 1000); // convert unix timestamp to Date object
      const day = date.toISOString().slice(0, 10); // get the yyyy-MM-dd part
      days.add(day);
    }
    return Array.from(days).sort(); // return a sorted array of dates
  }

  // get the medias that belong to a specific date
  getMediasByDay(medias: MediaItem[], day: string): MediaItem[] {
    return medias.filter((media) => {
      const date = new Date(media.time * 1000); // convert unix timestamp to Date object
      const mediaDay = date.toISOString().slice(0, 10); // get the yyyy-MM-dd part
      return mediaDay === day; // return true if the media date matches the given day
    });
  }
  back() {
    this.router.navigate(["/"]);
  }
  imgPreview(media: MediaItem) {
    return this.mediaService.getThumbnailUrl(media.id);
  }
}
