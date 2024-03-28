import { CommonModule } from "@angular/common";
import { Component, Input, OnInit } from "@angular/core";

@Component({
  selector: "lg-text-switcher",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./text-switcher.component.html",
  styles: ``,
})
export class TextSwitcherComponent implements OnInit {
  @Input() texts: string[] = [];
  currentString: string | undefined;
  private currentIndex: number = 0;
  private timer: any;

  ngOnInit(): void {
    this.startSwitching();
  }

  startSwitching(): void {
    this.timer = setInterval(() => {
      if (this.texts.length > 0) {
        this.currentString = this.texts[this.currentIndex];
        this.currentIndex = (this.currentIndex + 1) % this.texts.length;
      }
    }, 1000);
  }

  ngOnDestroy(): void {
    if (this.timer) {
      clearInterval(this.timer);
    }
  }
}
