> apiVersion = "apps/v1"

> kind = "Deployment"

> metadata > annotations = null

> metadata > clusterName = null

> metadata > creationTimestamp = null

> metadata > deletionGracePeriodSeconds = null

> metadata > deletionTimestamp = null

> metadata > finalizers = null

> metadata > generateName = null

> metadata > generation = null

> metadata > labels > ["app"] = "complex-app"

> metadata > name = "complex-deployment"

> metadata > namespace = null

> metadata > ownerReferences = null

> metadata > resourceVersion = null

> metadata > selfLink = null

> metadata > uid = null

> spec > minReadySeconds = null

> spec > paused = null

> spec > progressDeadlineSeconds = null

> spec > replicas = 3

> spec > revisionHistoryLimit = null

> spec > rollbackTo = null

> spec > selector > matchExpressions = null

> spec > selector > matchLabels > ["app"] = "complex-app"

> spec > strategy = null

> spec > template > metadata > annotations = null

> spec > template > metadata > clusterName = null

> spec > template > metadata > creationTimestamp = null

> spec > template > metadata > deletionGracePeriodSeconds = null

> spec > template > metadata > deletionTimestamp = null

> spec > template > metadata > finalizers = null

> spec > template > metadata > generateName = null

> spec > template > metadata > generation = null

> spec > template > metadata > labels > ["app"] = "complex-app"

> spec > template > metadata > name = null

> spec > template > metadata > namespace = null

> spec > template > metadata > ownerReferences = null

> spec > template > metadata > resourceVersion = null

> spec > template > metadata > selfLink = null

> spec > template > metadata > uid = null

> spec > template > spec > activeDeadlineSeconds = null

> spec > template > spec > affinity > nodeAffinity = null

> spec > template > spec > affinity > podAffinity > preferredDuringSchedulingIgnoredDuringExecution = null

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > key = "app"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > operator = "In"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > values = ["complex-app"]

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchLabels = null

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > namespaces = null

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > topologyKey = "kubernetes.io/hostname"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > labelSelector > matchExpressions > [0] > key = "zone"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > labelSelector > matchExpressions > [0] > operator = "In"

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > labelSelector > matchExpressions > [0] > values = ["zone1", "zone2"]

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > labelSelector > matchLabels = null

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > namespaces = null

> spec > template > spec > affinity > podAffinity > requiredDuringSchedulingIgnoredDuringExecution > [1] > topologyKey = "failure-domain.beta.kubernetes.io/zone"

> spec > template > spec > affinity > podAntiAffinity > preferredDuringSchedulingIgnoredDuringExecution = null

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > key = "app"

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > operator = "In"

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchExpressions > [0] > values = ["complex-app"]

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > labelSelector > matchLabels = null

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > namespaces = null

> spec > template > spec > affinity > podAntiAffinity > requiredDuringSchedulingIgnoredDuringExecution > [0] > topologyKey = "kubernetes.io/hostname"

> spec > template > spec > automountServiceAccountToken = null

> spec > template > spec > containers > [0] > args = null

> spec > template > spec > containers > [0] > command = null

> spec > template > spec > containers > [0] > env > [0] > name = "DB_HOST"

> spec > template > spec > containers > [0] > env > [0] > value = null

> spec > template > spec > containers > [0] > env > [0] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [0] > env > [0] > valueFrom > fieldRef = null

> spec > template > spec > containers > [0] > env > [0] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [0] > env > [0] > valueFrom > secretKeyRef > key = "db-host"

> spec > template > spec > containers > [0] > env > [0] > valueFrom > secretKeyRef > name = "db-secrets"

> spec > template > spec > containers > [0] > env > [0] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [0] > env > [1] > name = "DB_PORT"

> spec > template > spec > containers > [0] > env > [1] > value = null

> spec > template > spec > containers > [0] > env > [1] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [0] > env > [1] > valueFrom > fieldRef = null

> spec > template > spec > containers > [0] > env > [1] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [0] > env > [1] > valueFrom > secretKeyRef > key = "db-port"

> spec > template > spec > containers > [0] > env > [1] > valueFrom > secretKeyRef > name = "db-secrets"

> spec > template > spec > containers > [0] > env > [1] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [0] > env > [2] > name = "DB_USER"

> spec > template > spec > containers > [0] > env > [2] > value = null

> spec > template > spec > containers > [0] > env > [2] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [0] > env > [2] > valueFrom > fieldRef = null

> spec > template > spec > containers > [0] > env > [2] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > key = "db-user"

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > name = "db-secrets"

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [0] > env > [3] > name = "DB_PASSWORD"

> spec > template > spec > containers > [0] > env > [3] > value = null

> spec > template > spec > containers > [0] > env > [3] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [0] > env > [3] > valueFrom > fieldRef = null

> spec > template > spec > containers > [0] > env > [3] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > key = "db-password"

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > name = "db-secrets"

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [0] > env > [4] > name = "APP_CONFIG"

> spec > template > spec > containers > [0] > env > [4] > value = "{\"foo\": \"bar\", \"baz\": \"qux\"}"

> spec > template > spec > containers > [0] > env > [4] > valueFrom = null

> spec > template > spec > containers > [0] > envFrom = null

> spec > template > spec > containers > [0] > image = "myregistry.com/complex-app:latest"

> spec > template > spec > containers > [0] > imagePullPolicy = "Always"

> spec > template > spec > containers > [0] > lifecycle = null

> spec > template > spec > containers > [0] > livenessProbe > exec = null

> spec > template > spec > containers > [0] > livenessProbe > failureThreshold = 3

> spec > template > spec > containers > [0] > livenessProbe > httpGet > host = null

> spec > template > spec > containers > [0] > livenessProbe > httpGet > httpHeaders = null

> spec > template > spec > containers > [0] > livenessProbe > httpGet > path = "/healthz"

> spec > template > spec > containers > [0] > livenessProbe > httpGet > port = 8080

> spec > template > spec > containers > [0] > livenessProbe > httpGet > scheme = null

> spec > template > spec > containers > [0] > livenessProbe > initialDelaySeconds = 10

> spec > template > spec > containers > [0] > livenessProbe > periodSeconds = 5

> spec > template > spec > containers > [0] > livenessProbe > successThreshold = null

> spec > template > spec > containers > [0] > livenessProbe > tcpSocket = null

> spec > template > spec > containers > [0] > livenessProbe > timeoutSeconds = null

> spec > template > spec > containers > [0] > name = "main-container"

> spec > template > spec > containers > [0] > ports > [0] > containerPort = 8080

> spec > template > spec > containers > [0] > ports > [0] > hostIP = null

> spec > template > spec > containers > [0] > ports > [0] > hostPort = null

> spec > template > spec > containers > [0] > ports > [0] > name = null

> spec > template > spec > containers > [0] > ports > [0] > protocol = null

> spec > template > spec > containers > [0] > readinessProbe > exec = null

> spec > template > spec > containers > [0] > readinessProbe > failureThreshold = 3

> spec > template > spec > containers > [0] > readinessProbe > httpGet > host = null

> spec > template > spec > containers > [0] > readinessProbe > httpGet > httpHeaders = null

> spec > template > spec > containers > [0] > readinessProbe > httpGet > path = "/readyz"

> spec > template > spec > containers > [0] > readinessProbe > httpGet > port = 8080

> spec > template > spec > containers > [0] > readinessProbe > httpGet > scheme = null

> spec > template > spec > containers > [0] > readinessProbe > initialDelaySeconds = 10

> spec > template > spec > containers > [0] > readinessProbe > periodSeconds = 5

> spec > template > spec > containers > [0] > readinessProbe > successThreshold = null

> spec > template > spec > containers > [0] > readinessProbe > tcpSocket = null

> spec > template > spec > containers > [0] > readinessProbe > timeoutSeconds = null

> spec > template > spec > containers > [0] > resources = null

> spec > template > spec > containers > [0] > securityContext = null

> spec > template > spec > containers > [0] > stdin = null

> spec > template > spec > containers > [0] > stdinOnce = null

> spec > template > spec > containers > [0] > terminationMessagePath = null

> spec > template > spec > containers > [0] > terminationMessagePolicy = null

> spec > template > spec > containers > [0] > tty = null

> spec > template > spec > containers > [0] > volumeMounts > [0] > mountPath = "/etc/app-config"

> spec > template > spec > containers > [0] > volumeMounts > [0] > name = "config-volume"

> spec > template > spec > containers > [0] > volumeMounts > [0] > readOnly = null

> spec > template > spec > containers > [0] > volumeMounts > [0] > subPath = null

> spec > template > spec > containers > [0] > volumeMounts > [1] > mountPath = "/var/data"

> spec > template > spec > containers > [0] > volumeMounts > [1] > name = "data-volume"

> spec > template > spec > containers > [0] > volumeMounts > [1] > readOnly = null

> spec > template > spec > containers > [0] > volumeMounts > [1] > subPath = null

> spec > template > spec > containers > [0] > workingDir = null

> spec > template > spec > containers > [1] > args = null

> spec > template > spec > containers > [1] > command = null

> spec > template > spec > containers > [1] > env > [0] > name = "APP_CONFIG"

> spec > template > spec > containers > [1] > env > [0] > value = "{\"foo\": \"bar\", \"baz\": \"qux\"}"

> spec > template > spec > containers > [1] > env > [0] > valueFrom = null

> spec > template > spec > containers > [1] > envFrom = null

> spec > template > spec > containers > [1] > image = "myregistry.com/sidecar:latest"

> spec > template > spec > containers > [1] > imagePullPolicy = "Always"

> spec > template > spec > containers > [1] > lifecycle = null

> spec > template > spec > containers > [1] > livenessProbe = null

> spec > template > spec > containers > [1] > name = "sidecar-container"

> spec > template > spec > containers > [1] > ports = null

> spec > template > spec > containers > [1] > readinessProbe = null

> spec > template > spec > containers > [1] > resources = null

> spec > template > spec > containers > [1] > securityContext = null

> spec > template > spec > containers > [1] > stdin = null

> spec > template > spec > containers > [1] > stdinOnce = null

> spec > template > spec > containers > [1] > terminationMessagePath = null

> spec > template > spec > containers > [1] > terminationMessagePolicy = null

> spec > template > spec > containers > [1] > tty = null

> spec > template > spec > containers > [1] > volumeMounts > [0] > mountPath = "/etc/app-config"

> spec > template > spec > containers > [1] > volumeMounts > [0] > name = "config-volume"

> spec > template > spec > containers > [1] > volumeMounts > [0] > readOnly = null

> spec > template > spec > containers > [1] > volumeMounts > [0] > subPath = null

> spec > template > spec > containers > [1] > workingDir = null

> spec > template > spec > dnsPolicy = null

> spec > template > spec > hostIPC = null

> spec > template > spec > hostname = null

> spec > template > spec > hostNetwork = null

> spec > template > spec > hostPID = null

> spec > template > spec > imagePullSecrets > [0] > name = "registry-secret"

> spec > template > spec > initContainers = null

> spec > template > spec > nodeName = null

> spec > template > spec > nodeSelector > ["disktype"] = "ssd"

> spec > template > spec > restartPolicy = null

> spec > template > spec > schedulerName = null

> spec > template > spec > securityContext > fsGroup = 2000

> spec > template > spec > securityContext > runAsNonRoot = null

> spec > template > spec > securityContext > runAsUser = 1000

> spec > template > spec > securityContext > seLinuxOptions = null

> spec > template > spec > securityContext > supplementalGroups = null

> spec > template > spec > serviceAccount = null

> spec > template > spec > serviceAccountName = null

> spec > template > spec > subdomain = null

> spec > template > spec > terminationGracePeriodSeconds = null

> spec > template > spec > tolerations = null

> spec > template > spec > volumes > [0] > awsElasticBlockStore = null

> spec > template > spec > volumes > [0] > azureDisk = null

> spec > template > spec > volumes > [0] > azureFile = null

> spec > template > spec > volumes > [0] > cephfs = null

> spec > template > spec > volumes > [0] > cinder = null

> spec > template > spec > volumes > [0] > configMap > defaultMode = null

> spec > template > spec > volumes > [0] > configMap > items = null

> spec > template > spec > volumes > [0] > configMap > name = "app-config"

> spec > template > spec > volumes > [0] > configMap > optional = null

> spec > template > spec > volumes > [0] > downwardAPI = null

> spec > template > spec > volumes > [0] > emptyDir = null

> spec > template > spec > volumes > [0] > fc = null

> spec > template > spec > volumes > [0] > flexVolume = null

> spec > template > spec > volumes > [0] > flocker = null

> spec > template > spec > volumes > [0] > gcePersistentDisk = null

> spec > template > spec > volumes > [0] > gitRepo = null

> spec > template > spec > volumes > [0] > glusterfs = null

> spec > template > spec > volumes > [0] > hostPath = null

> spec > template > spec > volumes > [0] > iscsi = null

> spec > template > spec > volumes > [0] > name = "config-volume"

> spec > template > spec > volumes > [0] > nfs = null

> spec > template > spec > volumes > [0] > persistentVolumeClaim = null

> spec > template > spec > volumes > [0] > photonPersistentDisk = null

> spec > template > spec > volumes > [0] > portworxVolume = null

> spec > template > spec > volumes > [0] > projected = null

> spec > template > spec > volumes > [0] > quobyte = null

> spec > template > spec > volumes > [0] > rbd = null

> spec > template > spec > volumes > [0] > scaleIO = null

> spec > template > spec > volumes > [0] > secret = null

> spec > template > spec > volumes > [0] > vsphereVolume = null

> spec > template > spec > volumes > [1] > awsElasticBlockStore = null

> spec > template > spec > volumes > [1] > azureDisk = null

> spec > template > spec > volumes > [1] > azureFile = null

> spec > template > spec > volumes > [1] > cephfs = null

> spec > template > spec > volumes > [1] > cinder = null

> spec > template > spec > volumes > [1] > configMap = null

> spec > template > spec > volumes > [1] > downwardAPI = null

> spec > template > spec > volumes > [1] > emptyDir > medium = null

> spec > template > spec > volumes > [1] > fc = null

> spec > template > spec > volumes > [1] > flexVolume = null

> spec > template > spec > volumes > [1] > flocker = null

> spec > template > spec > volumes > [1] > gcePersistentDisk = null

> spec > template > spec > volumes > [1] > gitRepo = null

> spec > template > spec > volumes > [1] > glusterfs = null

> spec > template > spec > volumes > [1] > hostPath = null

> spec > template > spec > volumes > [1] > iscsi = null

> spec > template > spec > volumes > [1] > name = "data-volume"

> spec > template > spec > volumes > [1] > nfs = null

> spec > template > spec > volumes > [1] > persistentVolumeClaim = null

> spec > template > spec > volumes > [1] > photonPersistentDisk = null

> spec > template > spec > volumes > [1] > portworxVolume = null

> spec > template > spec > volumes > [1] > projected = null

> spec > template > spec > volumes > [1] > quobyte = null

> spec > template > spec > volumes > [1] > rbd = null

> spec > template > spec > volumes > [1] > scaleIO = null

> spec > template > spec > volumes > [1] > secret = null

> spec > template > spec > volumes > [1] > vsphereVolume = null

> status = null