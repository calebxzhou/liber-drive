import { Component, Input, OnInit } from '@angular/core';
import { FileItem } from '../file-item';
import { PageService } from '../page.service';



@Component({
  selector: 'dir-item',
  standalone: true,
  imports: [],
  templateUrl: './dir-item.component.html',
  styles: ``
})
export class DirItemComponent {


  @Input()
  fileItem!: FileItem;
  constructor(private page:PageService){}
  click(): void {
    this.page.goNextPath(this.fileItem.name);
  }
}
