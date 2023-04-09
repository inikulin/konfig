> apiVersion = "apps/v1"

> kind = "Deployment"

> metadata > labels > [app] = "my-app"

> metadata > name = "my-app"

> spec > replicas = 3

> spec > selector > matchLabels > [app] = "my-app"

> spec > template > metadata > labels > [app] = "my-app"

> spec > template > spec > containers > [0] > name = "frontend"

> spec > template > spec > containers > [0] > env > [0] > name = "DB_HOST"

> spec > template > spec > containers > [0] > env > [0] > value = "db-service"

> spec > template > spec > containers > [0] > env > [1] > name = "DB_PORT"

> spec > template > spec > containers > [0] > env > [1] > value = "5432"

> spec > template > spec > containers > [0] > env > [2] > name = "DB_USER"

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > key = "username"

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [0] > env > [3] > name = "DB_PASSWORD"

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > key = "password"

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [0] > image = "my-frontend-image:v1.0.0"

> spec > template > spec > containers > [0] > ports > [0] > containerPort = 80

> spec > template > spec > containers > [0] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

> spec > template > spec > containers > [0] > volumeMounts > [0] > name = "frontend-config"

> spec > template > spec > containers > [0] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

> spec > template > spec > containers > [0] > volumeMounts > [1] > name = "shared-data"

> spec > template > spec > containers > [1] > name = "backend"

> spec > template > spec > containers > [1] > env > [0] > name = "DB_HOST"

> spec > template > spec > containers > [1] > env > [0] > value = "db-service"

> spec > template > spec > containers > [1] > env > [1] > name = "DB_PORT"

> spec > template > spec > containers > [1] > env > [1] > value = "5432"

> spec > template > spec > containers > [1] > env > [2] > name = "DB_USER"

> spec > template > spec > containers > [1] > env > [2] > valueFrom > secretKeyRef > key = "username"

> spec > template > spec > containers > [1] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [1] > env > [3] > name = "DB_PASSWORD"

> spec > template > spec > containers > [1] > env > [3] > valueFrom > secretKeyRef > key = "password"

> spec > template > spec > containers > [1] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [1] > image = "my-backend-image:v1.0.0"

> spec > template > spec > containers > [1] > ports > [0] > containerPort = 8080

> spec > template > spec > containers > [1] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

> spec > template > spec > containers > [1] > volumeMounts > [0] > name = "backend-config"

> spec > template > spec > containers > [1] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

> spec > template > spec > containers > [1] > volumeMounts > [1] > name = "shared-data"

> spec > template > spec > containers > [2] > name = "job-worker"

> spec > template > spec > containers > [2] > env > [0] > name = "DB_HOST"

> spec > template > spec > containers > [2] > env > [0] > value = "db-service"

> spec > template > spec > containers > [2] > env > [1] > name = "DB_PORT"

> spec > template > spec > containers > [2] > env > [1] > value = "5432"

> spec > template > spec > containers > [2] > env > [2] > name = "DB_USER"

> spec > template > spec > containers > [2] > env > [2] > valueFrom > secretKeyRef > key = "username"

> spec > template > spec > containers > [2] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [2] > env > [3] > name = "DB_PASSWORD"

> spec > template > spec > containers > [2] > env > [3] > valueFrom > secretKeyRef > key = "password"

> spec > template > spec > containers > [2] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [2] > image = "my-job-worker-image:v1.0.0"

> spec > template > spec > containers > [2] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

> spec > template > spec > containers > [2] > volumeMounts > [0] > name = "job-worker-config"

> spec > template > spec > containers > [2] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

> spec > template > spec > containers > [2] > volumeMounts > [1] > name = "shared-data"

> spec > template > spec > volumes > [0] > name = "frontend-config"

> spec > template > spec > volumes > [0] > configMap > name = "frontend-config"

> spec > template > spec > volumes > [1] > name = "backend-config"

> spec > template > spec > volumes > [1] > configMap > name = "backend-config"

> spec > template > spec > volumes > [2] > name = "job-worker-config"

> spec > template > spec > volumes > [2] > configMap > name = "job-worker-config"

> spec > template > spec > volumes > [3] > name = "shared-data"

> spec > template > spec > volumes > [3] > persistentVolumeClaim > claimName = "shared-data-pvc"