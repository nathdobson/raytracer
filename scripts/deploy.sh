cargo build --release --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/server bins
docker build -t gcr.io/decent-micron-289607/raytracer .
docker push gcr.io/decent-micron-289607/raytracer
gcloud run deploy raytracer \
  --region us-west1 \
  --image gcr.io/decent-micron-289607/raytracer \
  --concurrency 1 \
  --cpu=8 \
  --memory=4Gi \
  --min-instances=0 \
  --max-instances=1
