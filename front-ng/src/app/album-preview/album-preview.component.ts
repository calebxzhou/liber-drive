import { Component, Input, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { AlbumInfo } from "../media/media";
import { CommonModule } from "@angular/common";
import { MatGridList, MatGridTile } from "@angular/material/grid-list";
import { toReadableSize } from "../util";
import {
  trigger,
  state,
  style,
  transition,
  animate,
} from "@angular/animations";
import { HttpEvent, HttpEventType } from "@angular/common/http";
import { ActivatedRoute, Router } from "@angular/router";

@Component({
  selector: "lg-album-preview",
  standalone: true,
  imports: [CommonModule, MatGridList, MatGridTile],
  templateUrl: "./album-preview.component.html",
})
export class AlbumPreviewComponent implements OnInit {
  @Input() info!: AlbumInfo;
  thumbnailUrl: string = "";
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private ms: MediaService
  ) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      this.ms.getAlbumTbnl(this.info.name).subscribe((e: HttpEvent<any>) => {
        if (e.type == HttpEventType.Response) {
          this.thumbnailUrl = URL.createObjectURL(e.body);
        }
      });
    });
  }
  goAlbum() {
    this.router.navigate([`/${this.info.name}`]);
  }
}
