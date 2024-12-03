
import { useState } from 'react';
import { Upload, Loader2, ArrowRight } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";

type ProcessedImage = {
  id: string;
  urls: {
    grayscale: string;
    corrupted: string;
    recovered: string;
  };
  timestamp: number;
};

const BASE_URL = 'http://15.207.111.49:8080';

const ImageProcessor = () => {
  const [images, setImages] = useState<ProcessedImage[]>([]);
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);

  const handleUpload = async (file: File) => {
    setLoading(true);
    setProgress(0);
    
    const formData = new FormData();
    formData.append('file', file);

    try {
      const response = await fetch(`${BASE_URL}/process`, {
        method: 'POST',
        body: formData,
      });
      
      const id = await response.json();

      
      // Simulate progress while images are being processed
      const progressInterval = setInterval(() => {
        setProgress(prev => {
          if (prev >= 90) {
            clearInterval(progressInterval);
            return 90;
          }
          return prev + 10;
        });
      }, 500);

      const newImage: ProcessedImage = {
        id,
        urls: {
          grayscale: `${BASE_URL}/images/grayscale-${id}.jpg`,
          corrupted: `${BASE_URL}/images/corrupted_image-${id}.png`,
          recovered: `${BASE_URL}/images/recovered_image-${id}.png`
        },
        timestamp: Date.now()
      };

      setImages(prev => [newImage, ...prev]);
      setProgress(100);
      
      setTimeout(() => {
        setProgress(0);
        setLoading(false);
      }, 500);
      
    } catch (error) {
      console.error('Upload failed:', error);
      setLoading(false);
      setProgress(0);
    }
  };

  return (
    <div className="flex justify-center align-middle p-6">
      <div className="max-w-6xl mx-auto space-y-8">
        <Card className="border-2 border-dashed border-gray-200 hover:border-primary/50 transition-colors">
          <CardContent className="pt-6">
            <input
              id="upload"
              type="file"
              className="hidden"
              onChange={(e) => e.target.files?.[0] && handleUpload(e.target.files[0])}
              accept="image/*"
            />
            <Button
              variant="outline"
              className="w-full h-40 bg-transparent"
              onClick={() => document.getElementById('upload')?.click()}
              disabled={loading}
            >
              <div className="flex flex-col items-center gap-4">
                {loading ? (
                  <>
                    <Loader2 className="h-8 w-8 animate-spin text-primary" />
                    <div className="w-full max-w-xs">
                      <Progress value={progress} className="h-2" />
                    </div>
                    <span className="text-sm text-muted-foreground">Processing image...</span>
                  </>
                ) : (
                  <>
                    <Upload className="h-8 w-8" />
                    <div className="space-y-2 text-center">
                      <p className="text-lg font-medium">Drop your image here or click to browse</p>
                      <p className="text-sm text-muted-foreground">Supports: JPG, PNG up to 10MB</p>
                    </div>
                  </>
                )}
              </div>
            </Button>
          </CardContent>
        </Card>

        <div className="grid grid-cols-1 gap-8">
          {images.map((image) => (
            <Card key={image.id} className="overflow-hidden">
              <CardHeader>
                <CardTitle className="text-sm text-muted-foreground">
                  Processed {new Date(image.timestamp).toLocaleString()}
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
                  <ImageCard title="Grayscale" src={image.urls.grayscale} />
                  
                  <ArrowRight className="hidden md:block mx-auto h-6 w-6 text-muted-foreground" />
                  <ImageCard title="Corrupted" src={image.urls.corrupted} />
                  <ArrowRight className="hidden md:block mx-auto h-6 w-6 text-muted-foreground" />
                  <ImageCard title="Recovered" src={image.urls.recovered} />
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
};

const ImageCard = ({ title, src }: { title: string; src: string }) => (
  <div className="space-y-2">
    <h3 className="font-medium text-center">{title}</h3>
    <div className="rounded-lg overflow-hidden border bg-white shadow-sm">
      <img 
        src={src} 
        alt={title} 
        className="w-full h-48 object-cover"
        loading="lazy"
      />
      <div>Go here if does not render {src}</div>
    </div>
  </div>
);


function App() {

  return (
    <>
      <ImageProcessor />
    </>
  )
}

export default App
