# Nova Cloud

Nova Cloud is a lightweight, personal cloud storage application that allows users to upload and manage files in a designated Google Drive folder. It features a modern, expressive user interface built with Vue.js and a robust backend powered by Rust.

## Features

- **Secure User Authentication**: Simple username/password system.
- **Google Drive Integration**: Each user gets a dedicated folder within a master Google Drive account.
- **File Management**: Upload and view files.
- **Modern UI/UX**: A beautiful, dark-themed interface with glassmorphism effects and fluid animations, built on the principles of Material Design 3 (Expressive).

## Tech Stack

- **Frontend**: Vue.js, Vite, Tailwind CSS, Motion One (for animations)
- **Backend**: Rust, Axum, Tokio
- **Cloud Storage**: Google Drive API

## Project Structure

```
/nova-cloud
├── nova-cloud-backend/     # Rust (Axum) backend
│   ├── src/
│   │   ├── main.rs         # Main application logic, routing
│   │   └── google_drive.rs # Google Drive API integration
│   ├── config/             # Configuration files
│   │   └── google_client_secret.json # (Required) Google Service Account Key
│   ├── database.json       # (Auto-generated) User database
│   └── Cargo.toml
└── nova-cloud-frontend/    # Vue.js frontend
    ├── src/
    │   ├── views/          # Vue components for each page (Login, Register, Main)
    │   └── ...
    └── ...
```

## Setup and Configuration

### 1. Backend (Rust)

**Prerequisites**:
- Rust and Cargo installed.
- A Google Cloud Platform (GCP) project.

**Configuration**:

1.  **Create a Google Service Account**:
    - Go to the [GCP Console](https://console.cloud.google.com/iam-admin/serviceaccounts).
    - Create a new service account. Grant it the "Owner" role for simplicity, or more granular permissions to the Google Drive API if preferred.
    - Create a JSON key for this service account and download it.

2.  **Place the Service Account Key**:
    - Place the downloaded JSON key file into the `nova-cloud-backend/config/` directory.
    - Rename the file to `google_client_secret.json`.

3.  **Create and Share the Parent Google Drive Folder**:
    - In your personal Google Drive, create a parent folder where all user data will be stored (e.g., `Nova-Cloud-Storage`).
    - Share this folder with the service account's email address (found inside the JSON key file as `client_email`). Grant it "Editor" permissions.
    - Open the shared folder and copy its **Folder ID** from the URL (`https://drive.google.com/drive/folders/THIS_IS_THE_ID`).

4.  **Update the Backend Code**:
    - Open `nova-cloud-backend/src/main.rs`.
    - In the `register_handler` function, find the line `parents: Some(vec!["YOUR_PARENT_FOLDER_ID".to_string()])`.
    - Replace `YOUR_PARENT_FOLDER_ID` with the actual ID you copied.

### 2. Frontend (Vue)

**Prerequisites**:
- Node.js and npm installed.

**Installation**:

Navigate to the frontend directory and install the dependencies:

```bash
cd nova-cloud-frontend
npm install
```

## How to Run

1.  **Start the Backend Server**:
    Open a terminal in the `nova-cloud-backend` directory and run:
    ```bash
    cargo run
    ```
    The server will start on `http://localhost:3000`.

2.  **Start the Frontend Development Server**:
    Open a second terminal in the `nova-cloud-frontend` directory and run:
    ```bash
    npm run dev
    ```
    The application will be available at `http://localhost:5173`.

3.  **Usage**:
    - Open your browser to `http://localhost:5173`.
    - Create a new account.
    - Log in with your new credentials.
    - Use the `+` button to upload files.
