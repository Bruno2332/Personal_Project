export type FieldError = { field: string; message: string };
export type AppError =

    | {
          type: "not_found" | "database" | "internal";
          message: string;
      }
    | { type: "validation"; errors: FieldError[] };
