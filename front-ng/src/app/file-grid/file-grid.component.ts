import { Component, OnInit } from "@angular/core";
import { NavigationEnd, Router, RouterModule } from "@angular/router";
import { FileGridService } from "../file-grid.service";
import { Location } from "@angular/common";
import { FileItem } from "../file-item";
import { CommonModule } from "@angular/common";
import { DirItemComponent } from "../dir-item/dir-item.component";
import { ImgItemComponent } from "../img-item/img-item.component";
import { PageService } from "../page.service";

@Component({
  selector: "file-grid",
  standalone: true,
  imports: [CommonModule, DirItemComponent, ImgItemComponent],
  templateUrl: "./file-grid.component.html",
  styles: ``,
})
export class FileGridComponent implements OnInit {
  fileItems: FileItem[] = [];

  constructor(private page: PageService, private service: FileGridService) {
     
  }
  ngOnInit(): void {
    this.service
      .fetchFileItems(this.page.getPathUrl())
      .subscribe((fileItems) => (this.fileItems = fileItems));

    this.page.path$.subscribe((newPath) => {
        this.service
      .fetchFileItems(this.page.getPathUrl())
      .subscribe((fileItems) => (this.fileItems = fileItems));
    });
  }
}
