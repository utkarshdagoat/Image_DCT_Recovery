# **VeriPhoto**

## **Our project for Digital Image Processing Course (ECN-316)**

### **Initial Ideation**
Our initial idea for the project included limited core aspects of digital image processing (DIP). To better emphasize the subject matter, we refined our approach to focus more on DIP concepts.

<img src="images/DIP.png" height="1200" width="900" alt="Initial Ideation Image">

---

## **Architecture**
The architecture of VeriPhoto is divided into two main parts

1. **Raw Decoding**  
2. **ZK-SNARKs Proof Generation**

---

### **1. Raw Decoding**
The process of transforming raw image data from camera sensors into a usable image format, ensuring maximum detail extraction and high-quality image processing.

- **Metadata Extraction**:
  - Extract essential metadata such as dimensions, compression methods, and camera settings.
  - Helps tailor the decoding process for each image file.

- **Handling Compression**:
  - Manage both lossless and lossy compression.
  - Apply standard decompression algorithms or camera-specific decoding methods.

- **Color Reconstruction**:
  - Decode Bayer pattern (mosaic of red, green, and blue pixels).
  - Use algorithms like demosaicing to reconstruct full-color images.

- **White Balance and Exposure Adjustment**:
  - Apply white balance corrections and exposure adjustments based on camera settings.
  - Ensure accurate color representation.

- **Image Reconstruction**:
  - We get the final PPM format and can encode it to standard formats
  - Reconstruct the raw data into standard formats like PNG or JPEG.
  - The final image is ready for further processing or display.

---

### **2. ZK-SNARKs Proof Generation**
Details forthcoming as this process involves cryptographic proof systems used for validating data integrity without revealing raw image data.

---

## **Why We Used Rust?**

We selected Rust for its many advantages:

- **Performance**:
  - Comparable to C and C++, offering low-level memory management.
  - Efficiently handles large raw image files without compromising speed.

- **Memory Safety**:
  - Guarantees memory safety without relying on garbage collection.
  - Ensures stable handling of complex memory and data operations in image processing.

- **Concurrency**:
  - Leverages Rust's concurrency model for parallel processing of large-scale image tasks.
  - Improves decoding speed and supports real-time processing.

- **Error Handling**:
  - Provides robust error handling with `Result` and `Option`.
  - Makes it easier to manage potentially corrupted or unsupported raw files.

- **Community and Libraries**:
  - Rust's growing ecosystem offers efficient libraries for image processing and cryptographic operations.
  - Facilitates building scalable tools like VeriPhoto.

---

## **Real-Life Use Cases**

- **Forensic Image Verification**:
  - Authenticate and validate raw image data integrity.

- **Digital Photography**:
  - Post-processing of raw photos for professional photographers.

- **Surveillance Systems**:
  - Enhance raw image handling in security and monitoring applications.

- **Medical Imaging**:
  - Efficient decoding and processing of raw medical images like X-rays and MRIs.

- **Scientific Research**:
  - Process large volumes of raw astronomical or microscopic image data.

- **Legal Evidence**:
  - Ensure the authenticity of digital image evidence in legal proceedings.

- **Blockchain Applications**:
  - Integrate ZK-SNARKs for proving image authenticity in decentralized systems.
