import { Component, Input, OnInit } from '@angular/core';
import { FileItem } from '../file-item';
import { HttpClient } from '@angular/common/http';
import { CommonModule } from '@angular/common';

//图片
@Component({
  selector: 'img-item',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './img-item.component.html',
  styles: ``
})
export class ImgItemComponent implements OnInit{
  imageData: string='';
  imageLoaded= false;
  @Input() fileItem!: FileItem;
	constructor(private http : HttpClient) {
  }
  ngOnInit(): void {
    this.fetchThumbnailImage();
  }
  fetchThumbnailImage(): void {
    this.http.get(this.fileItem.queryUrl+'?preview=3', { responseType: 'blob' }).subscribe({
      next: (image) => {
        const reader = new FileReader();
        reader.onloadend = () => {
          this.imageData = reader.result as string;
          this.imageLoaded = true;
        }
        reader.readAsDataURL(image);
      },
      error: (error) => {
        console.error('Error fetching image:', error);
      }
    });
  }
  
}
