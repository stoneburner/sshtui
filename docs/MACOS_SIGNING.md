# macOS signing and notarization (GitHub Actions)

The release workflow signs the macOS binary with your **Developer ID Application** certificate and **notarizes** it so users do not see Gatekeeper warnings. You need a paid Apple Developer account and six GitHub secrets.

---

## 1. Create the Developer ID Application certificate

1. Go to [Apple Developer – Certificates](https://developer.apple.com/account/resources/certificates/list).
2. Click **+** to add a certificate.
3. Under **Software**, choose **Developer ID Application** and continue.
4. Create a **Certificate Signing Request** (CSR) on your Mac:
   - Open **Keychain Access** → menu **Keychain Access** → **Certificate Assistant** → **Request a Certificate From a Certificate Authority**.
   - Email: your email. Common Name: e.g. `Developer ID Application: Your Name`. CA: **Saved to disk**.
   - Save the `.certSigningRequest` file.
5. Upload the CSR in the Apple Developer page and download the `.cer` file.
6. Double‑click the `.cer` to add it to your **login** keychain.
7. In **Keychain Access**, find the certificate **Developer ID Application: …** (it may be under “My Certificates”). Note the **exact** name (including team ID in parentheses).

---

## 2. Export the certificate as .p12

1. In Keychain Access, select the **Developer ID Application** certificate (and its private key if listed separately).
2. Right‑click → **Export "…"**.
3. Save as **Certificate.p12**. Choose a **strong password** (you will store it in GitHub).
4. Convert the `.p12` to base64 for the secret (on your Mac):
   ```bash
   base64 -i Certificate.p12 -o Certificate.p12.txt
   ```
5. Copy the **entire** contents of `Certificate.p12.txt` (single line) for the secret `MACOS_CERTIFICATE_P12_BASE64`.

---

## 3. Create an App Store Connect API key (for notarization)

1. Go to [App Store Connect](https://appstoreconnect.apple.com) → **Users and Access** → **Keys** (under Integrations).
2. Click **+** to create a key. Name it e.g. `Notary API Key`. Access: **App Manager** or **Admin**.
3. Create the key, then **Download** the `.p8` file once (you cannot download it again).
4. Note:
   - **Key ID** (e.g. `ABCDE12345`)
   - **Issuer ID** (top of the Keys page, e.g. `12345678-1234-1234-1234-123456789012`)
5. Open the `.p8` file and copy its **full contents** (including `-----BEGIN PRIVATE KEY-----` and `-----END PRIVATE KEY-----`) for the secret `APPLE_NOTARY_KEY`.

---

## 4. Add GitHub secrets

In your repo: **Settings** → **Secrets and variables** → **Actions** → **New repository secret**. Add:

| Secret name | Value | From step |
|-------------|--------|-----------|
| `MACOS_CERTIFICATE_P12_BASE64` | Entire output of `base64 -i Certificate.p12` | Step 2 |
| `MACOS_CERTIFICATE_PASSWORD` | Password you set when exporting the .p12 | Step 2 |
| `MACOS_SIGNING_IDENTITY` | Exact name of the cert, e.g. `Developer ID Application: Your Name (TEAMID)` | Step 1 |
| `APPLE_NOTARY_KEY_ID` | Key ID of the App Store Connect API key | Step 3 |
| `APPLE_NOTARY_ISSUER_ID` | Issuer ID from App Store Connect | Step 3 |
| `APPLE_NOTARY_KEY` | Full contents of the `.p8` file | Step 3 |

---

## 5. What the workflow does

- **macOS job:** Builds the binary, imports the .p12, signs `sshtui` with `codesign`, creates a zip, submits it to Apple for notarization, staples the notarization ticket to the zip, and uploads the zip as the macOS release asset.
- **Release:** The macOS asset is `sshtui-aarch64-apple-darwin.zip` (signed and notarized). Linux assets stay as `.tar.gz`.

---

## 6. Optional: signing only (no notarization)

If you do not set the three `APPLE_NOTARY_*` secrets, the **Notarize (macOS)** step will fail. To ship only a **signed** binary (no notarization):

1. In `.github/workflows/release.yml`, remove or comment out the **Notarize (macOS)** step.
2. Change **Prepare archive (macOS)** so it does not create the zip (or keep the zip for local use only).
3. Change **Upload artifact (macOS)** back to uploading `${{ matrix.artifact }}.tar.gz` instead of `.zip`.
4. In **Prepare release assets**, copy only `*.tar.gz` (or adjust so the macOS artifact is the tarball again).
5. In the Formula, point macOS back to the `.tar.gz` URL and its SHA256.

Then only the three `MACOS_*` secrets are required.
