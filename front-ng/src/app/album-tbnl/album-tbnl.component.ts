import { Component, Input, OnInit } from "@angular/core";
import { AlbumInfo } from "../media/media";
import { CommonModule } from "@angular/common";
import { ActivatedRoute, Router, RouterModule } from "@angular/router";
import { MediaService } from "../media/media.service";
import { ImageTbnlComponent } from "../image-tbnl/image-tbnl.component";

@Component({
  selector: "lg-album-tbnl",
  standalone: true,
  imports: [CommonModule, RouterModule, ImageTbnlComponent],
  templateUrl: "./album-tbnl.component.html",
  styles: `  
  .gallery {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  grid-template-rows: repeat(2,64px); 
  overflow:hidden;
  gap: 4px; /* Adjust the gap between images if needed */
}
   
  `,
})
export class AlbumTbnlComponent implements OnInit {
  @Input() album!: AlbumInfo;
  path: string = "";
  //= () =>  (this.album.parent ? this.album.parent + "/" : "") + this.album.name;
  constructor(private route: ActivatedRoute, private ms: MediaService) {}
  ngOnInit(): void {
    const pathNow = this.route.snapshot.queryParams["path"];
    if (pathNow) {
      this.path = pathNow + "/" + this.album.name;
    } else {
      this.path = this.album.name;
    }
  }
  goAlbum(name: string) {
    this.ms.goAlbum(name);
  }
}
