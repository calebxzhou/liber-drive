import { CommonModule } from "@angular/common";
import {
  AfterContentInit,
  AfterRenderPhase,
  AfterViewChecked,
  AfterViewInit,
  Component,
  Input,
  OnInit,
} from "@angular/core";
import { ImageExif } from "../media/media";

@Component({
  selector: "lg-exif-display",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./exif-display.component.html",
  styles: ``,
})
export class ExifDisplayComponent implements AfterViewChecked {
  @Input() exif!: ImageExif | undefined;
  ngAfterViewChecked(): void {
    //console.log(this.exif);
  }
  focalLen(): string {
    let exif = this.exif;
    if (exif) {
      if (exif.lens.includes("EF-S") || exif.lens.includes("RF-S")) {
        return Math.round(Number(exif.focal_len) * 1.6) + "";
      }
      return exif.focal_len ?? "";
    }
    return "";
  }
  cameraMake(): string {
    return (
      this.exif?.make
        .replaceAll("Canon", "佳能")
        .replaceAll("NIKON", "尼康")
        .replaceAll("SONY", "索尼")
        .replaceAll("Panasonic", "松下") ?? ""
    );
  }
}
