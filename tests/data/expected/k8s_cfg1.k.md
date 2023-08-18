> apiVersion = "apps/v1"

> kind = "Deployment"

> metadata > labels > [app] = "complex-app"

> metadata > name = "complex-deployment"

> spec > replicas = 3

> spec > selector > matchLabels > [app] = "complex-app"

> spec > template > metadata > labels > [app] = "complex-app"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > key = "app"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > operator = "In"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > values = ["complex-app"]

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > topologyKey = "kubernetes.io/hostname"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > labelSelector > matchExpressions > [0] > key = "zone"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > labelSelector > matchExpressions > [0] > operator = "In"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > labelSelector > matchExpressions > [0] > values = ["zone1", "zone2"]

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > topologyKey = "failure-domain.beta.kubernetes.io/zone"

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > key = "app"

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > operator = "In"

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > values = ["complex-app"]

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > topologyKey = "kubernetes.io/hostname"

> spec > template > spec > containers > [0] > env > [0] > name = "DB_HOST"

> spec > template > spec > containers > [0] > env > [0] > valueFrom > secretKeyRef > key = "db-host"

> spec > template > spec > containers > [0] > env > [0] > valueFrom > secretKeyRef > name = "db-secrets"

> spec > template > spec > containers > [0] > env > [1] > name = "DB_PORT"

> spec > template > spec > containers > [0] > env > [1] > valueFrom > secretKeyRef > key = "db-port"

> spec > template > spec > containers > [0] > env > [1] > valueFrom > secretKeyRef > name = "db-secrets"

> spec > template > spec > containers > [0] > env > [2] > name = "DB_USER"

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > key = "db-user"

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > name = "db-secrets"

> spec > template > spec > containers > [0] > env > [3] > name = "DB_PASSWORD"

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > key = "db-password"

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > name = "db-secrets"

> spec > template > spec > containers > [0] > env > [4] > name = "APP_CONFIG"

> spec > template > spec > containers > [0] > env > [4] > value = "{\"foo\": \"bar\", \"baz\": \"qux\"}"

> spec > template > spec > containers > [0] > image = "myregistry.com/complex-app:latest"

> spec > template > spec > containers > [0] > imagePullPolicy = "Always"

> spec > template > spec > containers > [0] > livenessProbe > failureThreshold = 3

> spec > template > spec > containers > [0] > livenessProbe > httpGet > path = "/healthz"

> spec > template > spec > containers > [0] > livenessProbe > httpGet > port = 8080

> spec > template > spec > containers > [0] > livenessProbe > initialDelaySeconds = 10

> spec > template > spec > containers > [0] > livenessProbe > periodSeconds = 5

> spec > template > spec > containers > [0] > name = "main-container"

> spec > template > spec > containers > [0] > ports > [0] > containerPort = 8080

> spec > template > spec > containers > [0] > readinessProbe > failureThreshold = 3

> spec > template > spec > containers > [0] > readinessProbe > httpGet > path = "/readyz"

> spec > template > spec > containers > [0] > readinessProbe > httpGet > port = 8080

> spec > template > spec > containers > [0] > readinessProbe > initialDelaySeconds = 10

> spec > template > spec > containers > [0] > readinessProbe > periodSeconds = 5

> spec > template > spec > containers > [0] > volumeMounts > [0] > mountPath = "/etc/app-config"

> spec > template > spec > containers > [0] > volumeMounts > [0] > name = "config-volume"

> spec > template > spec > containers > [0] > volumeMounts > [1] > mountPath = "/var/data"

> spec > template > spec > containers > [0] > volumeMounts > [1] > name = "data-volume"

> spec > template > spec > containers > [1] > env > [0] > name = "APP_CONFIG"

> spec > template > spec > containers > [1] > env > [0] > value = "{\"foo\": \"bar\", \"baz\": \"qux\"}"

> spec > template > spec > containers > [1] > image = "myregistry.com/sidecar:latest"

> spec > template > spec > containers > [1] > imagePullPolicy = "Always"

> spec > template > spec > containers > [1] > name = "sidecar-container"

> spec > template > spec > containers > [1] > volumeMounts > [0] > mountPath = "/etc/app-config"

> spec > template > spec > containers > [1] > volumeMounts > [0] > name = "config-volume"

> spec > template > spec > imagePullSecrets > [0] > name = "registry-secret"

> spec > template > spec > nodeSelector > [disktype] = "ssd"

> spec > template > spec > securityContext > fsGroup = 2000

> spec > template > spec > securityContext > runAsUser = 1000

> spec > template > spec > volumes > [0] > configMap > name = "app-config"

> spec > template > spec > volumes > [0] > name = "config-volume"

> spec > template > spec > volumes > [1] > name = "data-volume"