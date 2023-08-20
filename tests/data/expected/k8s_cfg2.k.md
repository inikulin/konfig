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

> metadata > labels > ["app"] = "my-app"

> metadata > name = "my-app"

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

> spec > selector > matchLabels > ["app"] = "my-app"

> spec > strategy = null

> spec > template > metadata > annotations = null

> spec > template > metadata > clusterName = null

> spec > template > metadata > creationTimestamp = null

> spec > template > metadata > deletionGracePeriodSeconds = null

> spec > template > metadata > deletionTimestamp = null

> spec > template > metadata > finalizers = null

> spec > template > metadata > generateName = null

> spec > template > metadata > generation = null

> spec > template > metadata > labels > ["app"] = "my-app"

> spec > template > metadata > name = null

> spec > template > metadata > namespace = null

> spec > template > metadata > ownerReferences = null

> spec > template > metadata > resourceVersion = null

> spec > template > metadata > selfLink = null

> spec > template > metadata > uid = null

> spec > template > spec > activeDeadlineSeconds = null

> spec > template > spec > affinity = null

> spec > template > spec > automountServiceAccountToken = null

> spec > template > spec > containers > [0] > args = null

> spec > template > spec > containers > [0] > command = null

> spec > template > spec > containers > [0] > env > [0] > name = "DB_HOST"

> spec > template > spec > containers > [0] > env > [0] > value = "db-service"

> spec > template > spec > containers > [0] > env > [0] > valueFrom = null

> spec > template > spec > containers > [0] > env > [1] > name = "DB_PORT"

> spec > template > spec > containers > [0] > env > [1] > value = "5432"

> spec > template > spec > containers > [0] > env > [1] > valueFrom = null

> spec > template > spec > containers > [0] > env > [2] > name = "DB_USER"

> spec > template > spec > containers > [0] > env > [2] > value = null

> spec > template > spec > containers > [0] > env > [2] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [0] > env > [2] > valueFrom > fieldRef = null

> spec > template > spec > containers > [0] > env > [2] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > key = "username"

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [0] > env > [3] > name = "DB_PASSWORD"

> spec > template > spec > containers > [0] > env > [3] > value = null

> spec > template > spec > containers > [0] > env > [3] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [0] > env > [3] > valueFrom > fieldRef = null

> spec > template > spec > containers > [0] > env > [3] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > key = "password"

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [0] > envFrom = null

> spec > template > spec > containers > [0] > image = "my-frontend-image:v1.0.0"

> spec > template > spec > containers > [0] > imagePullPolicy = null

> spec > template > spec > containers > [0] > lifecycle = null

> spec > template > spec > containers > [0] > livenessProbe = null

> spec > template > spec > containers > [0] > name = "frontend"

> spec > template > spec > containers > [0] > ports > [0] > containerPort = 80

> spec > template > spec > containers > [0] > ports > [0] > hostIP = null

> spec > template > spec > containers > [0] > ports > [0] > hostPort = null

> spec > template > spec > containers > [0] > ports > [0] > name = null

> spec > template > spec > containers > [0] > ports > [0] > protocol = null

> spec > template > spec > containers > [0] > readinessProbe = null

> spec > template > spec > containers > [0] > resources = null

> spec > template > spec > containers > [0] > securityContext = null

> spec > template > spec > containers > [0] > stdin = null

> spec > template > spec > containers > [0] > stdinOnce = null

> spec > template > spec > containers > [0] > terminationMessagePath = null

> spec > template > spec > containers > [0] > terminationMessagePolicy = null

> spec > template > spec > containers > [0] > tty = null

> spec > template > spec > containers > [0] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

> spec > template > spec > containers > [0] > volumeMounts > [0] > name = "frontend-config"

> spec > template > spec > containers > [0] > volumeMounts > [0] > readOnly = null

> spec > template > spec > containers > [0] > volumeMounts > [0] > subPath = null

> spec > template > spec > containers > [0] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

> spec > template > spec > containers > [0] > volumeMounts > [1] > name = "shared-data"

> spec > template > spec > containers > [0] > volumeMounts > [1] > readOnly = null

> spec > template > spec > containers > [0] > volumeMounts > [1] > subPath = null

> spec > template > spec > containers > [0] > workingDir = null

> spec > template > spec > containers > [1] > args = null

> spec > template > spec > containers > [1] > command = null

> spec > template > spec > containers > [1] > env > [0] > name = "DB_HOST"

> spec > template > spec > containers > [1] > env > [0] > value = "db-service"

> spec > template > spec > containers > [1] > env > [0] > valueFrom = null

> spec > template > spec > containers > [1] > env > [1] > name = "DB_PORT"

> spec > template > spec > containers > [1] > env > [1] > value = "5432"

> spec > template > spec > containers > [1] > env > [1] > valueFrom = null

> spec > template > spec > containers > [1] > env > [2] > name = "DB_USER"

> spec > template > spec > containers > [1] > env > [2] > value = null

> spec > template > spec > containers > [1] > env > [2] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [1] > env > [2] > valueFrom > fieldRef = null

> spec > template > spec > containers > [1] > env > [2] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [1] > env > [2] > valueFrom > secretKeyRef > key = "username"

> spec > template > spec > containers > [1] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [1] > env > [2] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [1] > env > [3] > name = "DB_PASSWORD"

> spec > template > spec > containers > [1] > env > [3] > value = null

> spec > template > spec > containers > [1] > env > [3] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [1] > env > [3] > valueFrom > fieldRef = null

> spec > template > spec > containers > [1] > env > [3] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [1] > env > [3] > valueFrom > secretKeyRef > key = "password"

> spec > template > spec > containers > [1] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [1] > env > [3] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [1] > envFrom = null

> spec > template > spec > containers > [1] > image = "my-backend-image:v1.0.0"

> spec > template > spec > containers > [1] > imagePullPolicy = null

> spec > template > spec > containers > [1] > lifecycle = null

> spec > template > spec > containers > [1] > livenessProbe = null

> spec > template > spec > containers > [1] > name = "backend"

> spec > template > spec > containers > [1] > ports > [0] > containerPort = 8080

> spec > template > spec > containers > [1] > ports > [0] > hostIP = null

> spec > template > spec > containers > [1] > ports > [0] > hostPort = null

> spec > template > spec > containers > [1] > ports > [0] > name = null

> spec > template > spec > containers > [1] > ports > [0] > protocol = null

> spec > template > spec > containers > [1] > readinessProbe = null

> spec > template > spec > containers > [1] > resources = null

> spec > template > spec > containers > [1] > securityContext = null

> spec > template > spec > containers > [1] > stdin = null

> spec > template > spec > containers > [1] > stdinOnce = null

> spec > template > spec > containers > [1] > terminationMessagePath = null

> spec > template > spec > containers > [1] > terminationMessagePolicy = null

> spec > template > spec > containers > [1] > tty = null

> spec > template > spec > containers > [1] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

> spec > template > spec > containers > [1] > volumeMounts > [0] > name = "backend-config"

> spec > template > spec > containers > [1] > volumeMounts > [0] > readOnly = null

> spec > template > spec > containers > [1] > volumeMounts > [0] > subPath = null

> spec > template > spec > containers > [1] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

> spec > template > spec > containers > [1] > volumeMounts > [1] > name = "shared-data"

> spec > template > spec > containers > [1] > volumeMounts > [1] > readOnly = null

> spec > template > spec > containers > [1] > volumeMounts > [1] > subPath = null

> spec > template > spec > containers > [1] > workingDir = null

> spec > template > spec > containers > [2] > args = null

> spec > template > spec > containers > [2] > command = null

> spec > template > spec > containers > [2] > env > [0] > name = "DB_HOST"

> spec > template > spec > containers > [2] > env > [0] > value = "db-service"

> spec > template > spec > containers > [2] > env > [0] > valueFrom = null

> spec > template > spec > containers > [2] > env > [1] > name = "DB_PORT"

> spec > template > spec > containers > [2] > env > [1] > value = "5432"

> spec > template > spec > containers > [2] > env > [1] > valueFrom = null

> spec > template > spec > containers > [2] > env > [2] > name = "DB_USER"

> spec > template > spec > containers > [2] > env > [2] > value = null

> spec > template > spec > containers > [2] > env > [2] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [2] > env > [2] > valueFrom > fieldRef = null

> spec > template > spec > containers > [2] > env > [2] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [2] > env > [2] > valueFrom > secretKeyRef > key = "username"

> spec > template > spec > containers > [2] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [2] > env > [2] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [2] > env > [3] > name = "DB_PASSWORD"

> spec > template > spec > containers > [2] > env > [3] > value = null

> spec > template > spec > containers > [2] > env > [3] > valueFrom > configMapKeyRef = null

> spec > template > spec > containers > [2] > env > [3] > valueFrom > fieldRef = null

> spec > template > spec > containers > [2] > env > [3] > valueFrom > resourceFieldRef = null

> spec > template > spec > containers > [2] > env > [3] > valueFrom > secretKeyRef > key = "password"

> spec > template > spec > containers > [2] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

> spec > template > spec > containers > [2] > env > [3] > valueFrom > secretKeyRef > optional = null

> spec > template > spec > containers > [2] > envFrom = null

> spec > template > spec > containers > [2] > image = "my-job-worker-image:v1.0.0"

> spec > template > spec > containers > [2] > imagePullPolicy = null

> spec > template > spec > containers > [2] > lifecycle = null

> spec > template > spec > containers > [2] > livenessProbe = null

> spec > template > spec > containers > [2] > name = "job-worker"

> spec > template > spec > containers > [2] > ports = null

> spec > template > spec > containers > [2] > readinessProbe = null

> spec > template > spec > containers > [2] > resources = null

> spec > template > spec > containers > [2] > securityContext = null

> spec > template > spec > containers > [2] > stdin = null

> spec > template > spec > containers > [2] > stdinOnce = null

> spec > template > spec > containers > [2] > terminationMessagePath = null

> spec > template > spec > containers > [2] > terminationMessagePolicy = null

> spec > template > spec > containers > [2] > tty = null

> spec > template > spec > containers > [2] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

> spec > template > spec > containers > [2] > volumeMounts > [0] > name = "job-worker-config"

> spec > template > spec > containers > [2] > volumeMounts > [0] > readOnly = null

> spec > template > spec > containers > [2] > volumeMounts > [0] > subPath = null

> spec > template > spec > containers > [2] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

> spec > template > spec > containers > [2] > volumeMounts > [1] > name = "shared-data"

> spec > template > spec > containers > [2] > volumeMounts > [1] > readOnly = null

> spec > template > spec > containers > [2] > volumeMounts > [1] > subPath = null

> spec > template > spec > containers > [2] > workingDir = null

> spec > template > spec > dnsPolicy = null

> spec > template > spec > hostIPC = null

> spec > template > spec > hostname = null

> spec > template > spec > hostNetwork = null

> spec > template > spec > hostPID = null

> spec > template > spec > imagePullSecrets = null

> spec > template > spec > initContainers = null

> spec > template > spec > nodeName = null

> spec > template > spec > nodeSelector = null

> spec > template > spec > restartPolicy = null

> spec > template > spec > schedulerName = null

> spec > template > spec > securityContext = null

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

> spec > template > spec > volumes > [0] > configMap > name = "frontend-config"

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

> spec > template > spec > volumes > [0] > name = "frontend-config"

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

> spec > template > spec > volumes > [1] > configMap > defaultMode = null

> spec > template > spec > volumes > [1] > configMap > items = null

> spec > template > spec > volumes > [1] > configMap > name = "backend-config"

> spec > template > spec > volumes > [1] > configMap > optional = null

> spec > template > spec > volumes > [1] > downwardAPI = null

> spec > template > spec > volumes > [1] > emptyDir = null

> spec > template > spec > volumes > [1] > fc = null

> spec > template > spec > volumes > [1] > flexVolume = null

> spec > template > spec > volumes > [1] > flocker = null

> spec > template > spec > volumes > [1] > gcePersistentDisk = null

> spec > template > spec > volumes > [1] > gitRepo = null

> spec > template > spec > volumes > [1] > glusterfs = null

> spec > template > spec > volumes > [1] > hostPath = null

> spec > template > spec > volumes > [1] > iscsi = null

> spec > template > spec > volumes > [1] > name = "backend-config"

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

> spec > template > spec > volumes > [2] > awsElasticBlockStore = null

> spec > template > spec > volumes > [2] > azureDisk = null

> spec > template > spec > volumes > [2] > azureFile = null

> spec > template > spec > volumes > [2] > cephfs = null

> spec > template > spec > volumes > [2] > cinder = null

> spec > template > spec > volumes > [2] > configMap > defaultMode = null

> spec > template > spec > volumes > [2] > configMap > items = null

> spec > template > spec > volumes > [2] > configMap > name = "job-worker-config"

> spec > template > spec > volumes > [2] > configMap > optional = null

> spec > template > spec > volumes > [2] > downwardAPI = null

> spec > template > spec > volumes > [2] > emptyDir = null

> spec > template > spec > volumes > [2] > fc = null

> spec > template > spec > volumes > [2] > flexVolume = null

> spec > template > spec > volumes > [2] > flocker = null

> spec > template > spec > volumes > [2] > gcePersistentDisk = null

> spec > template > spec > volumes > [2] > gitRepo = null

> spec > template > spec > volumes > [2] > glusterfs = null

> spec > template > spec > volumes > [2] > hostPath = null

> spec > template > spec > volumes > [2] > iscsi = null

> spec > template > spec > volumes > [2] > name = "job-worker-config"

> spec > template > spec > volumes > [2] > nfs = null

> spec > template > spec > volumes > [2] > persistentVolumeClaim = null

> spec > template > spec > volumes > [2] > photonPersistentDisk = null

> spec > template > spec > volumes > [2] > portworxVolume = null

> spec > template > spec > volumes > [2] > projected = null

> spec > template > spec > volumes > [2] > quobyte = null

> spec > template > spec > volumes > [2] > rbd = null

> spec > template > spec > volumes > [2] > scaleIO = null

> spec > template > spec > volumes > [2] > secret = null

> spec > template > spec > volumes > [2] > vsphereVolume = null

> spec > template > spec > volumes > [3] > awsElasticBlockStore = null

> spec > template > spec > volumes > [3] > azureDisk = null

> spec > template > spec > volumes > [3] > azureFile = null

> spec > template > spec > volumes > [3] > cephfs = null

> spec > template > spec > volumes > [3] > cinder = null

> spec > template > spec > volumes > [3] > configMap = null

> spec > template > spec > volumes > [3] > downwardAPI = null

> spec > template > spec > volumes > [3] > emptyDir = null

> spec > template > spec > volumes > [3] > fc = null

> spec > template > spec > volumes > [3] > flexVolume = null

> spec > template > spec > volumes > [3] > flocker = null

> spec > template > spec > volumes > [3] > gcePersistentDisk = null

> spec > template > spec > volumes > [3] > gitRepo = null

> spec > template > spec > volumes > [3] > glusterfs = null

> spec > template > spec > volumes > [3] > hostPath = null

> spec > template > spec > volumes > [3] > iscsi = null

> spec > template > spec > volumes > [3] > name = "shared-data"

> spec > template > spec > volumes > [3] > nfs = null

> spec > template > spec > volumes > [3] > persistentVolumeClaim > claimName = "shared-data-pvc"

> spec > template > spec > volumes > [3] > persistentVolumeClaim > readOnly = null

> spec > template > spec > volumes > [3] > photonPersistentDisk = null

> spec > template > spec > volumes > [3] > portworxVolume = null

> spec > template > spec > volumes > [3] > projected = null

> spec > template > spec > volumes > [3] > quobyte = null

> spec > template > spec > volumes > [3] > rbd = null

> spec > template > spec > volumes > [3] > scaleIO = null

> spec > template > spec > volumes > [3] > secret = null

> spec > template > spec > volumes > [3] > vsphereVolume = null

> status = null