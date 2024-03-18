Creating a Postgres Extension with Rust, PGRX
=============================================

### Steps:

1. Define the functions for the extension in the `lib.rs` file.
   
2. Execute the `make package` command to generate the extension files.
   
3. Move the generated files to their respective directories within the local PostgreSQL installation.

4. Use the following SQL commands to manage the extension:
    ```sql
    DROP EXTENSION IF EXISTS my_extension;
    CREATE EXTENSION my_extension;
    ```

5. To test the extension, execute the following SQL command:
    ```sql
    SELECT m.id, welcome_extension(m."name")
    FROM public.mock_data AS m
    LIMIT 10;
    ```
