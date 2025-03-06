import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { MatCommonModule } from "@angular/material/core";
import { MatDialogModule, MatDialogRef } from "@angular/material/dialog";
import { MatSelectModule } from "@angular/material/select";
import { MatInputModule } from "@angular/material/input";
import { MatFormFieldModule } from "@angular/material/form-field";
import { ActivatedRoute, Router } from "@angular/router";
import { MatButtonModule } from "@angular/material/button";
@Component({
  selector: "lg-password-dialog",
  standalone: true,
  imports: [
    MatCommonModule,
    MatDialogModule,
    MatInputModule,
    MatButtonModule,
    CommonModule,
    FormsModule,
    MatFormFieldModule,
  ],
  templateUrl: "./password-dialog.component.html",
  styles: ``,
})
export class PasswordDialogComponent {
  password: string = "";

  constructor(
    public dialogRef: MatDialogRef<PasswordDialogComponent>,
    private route: ActivatedRoute,
    private router: Router
  ) {}

  onCancel(): void {
    this.dialogRef.close();
    this.router.navigate(["../"], { relativeTo: this.route });
  }

  onSubmit(): void {
    this.dialogRef.close(this.password);
  }
}
