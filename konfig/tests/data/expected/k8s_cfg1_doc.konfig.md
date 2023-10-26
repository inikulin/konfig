# APIVersion defines the versioned schema of this representation of an object.

 Servers
 should convert recognized schemas to the latest internal value, and may reject
 unrecognized values. More info:
 http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#resources

> apiVersion = "apps/v1"

# Kind is a string value representing the REST resource this object represents.

 Servers may
 infer this from the endpoint the client submits requests to. Cannot be updated. In
 CamelCase. More info:
 http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#types-kinds

> kind = "Deployment"

<h1>
ObjectMeta is metadata that all persisted resources must have, which includes all objectsusers must create.
</h1>

<h2>
Annotations is an unstructured key value map stored with a resource that may be set byexternal tools to store and retrieve arbitrary metadata.
</h2>

 They are not queryable and
 should be preserved when modifying objects. More info:
 http://kubernetes.io/docs/user-guide/annotations

> metadata > annotations = null

## The name of the cluster which the object belongs to.

 This is used to distinguish
 resources with same name and namespace in different clusters. This field is not set
 anywhere right now and apiserver is going to ignore it if set in create or update request.

> metadata > clusterName = null

> metadata > creationTimestamp = null

<h2>
Number of seconds allowed for this object to gracefully terminate before it will beremoved from the system.
</h2>

 Only set when deletionTimestamp is also set. May only be
 shortened. Read-only.

> metadata > deletionGracePeriodSeconds = null

> metadata > deletionTimestamp = null

## Must be empty before the object is deleted from the registry.

 Each entry is an identifier
 for the responsible component that will remove the entry from the list. If the
 deletionTimestamp of the object is non-nil, entries in this list can only be removed.

> metadata > finalizers = null

<h2>
GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IFthe Name field has not been provided.
</h2>

 If this field is used, the name returned to the
 client will be different than the name passed. This value will also be combined with a
 unique suffix. The provided value has the same validation rules as the Name field, and
 may be truncated by the length of the suffix required to make the value unique on the
 server.

 If this field is specified and the generated name exists, the server will NOT return a
 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout
 indicating a unique name could not be found in the time allotted, and the client should
 retry (optionally after the time indicated in the Retry-After header).

 Applied only if Name is not specified. More info:
 http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#idempotency

> metadata > generateName = null

## A sequence number representing a specific generation of the desired state.

 Populated by
 the system. Read-only.

> metadata > generation = null

<h2>
Map of string keys and values that can be used to organize and categorize (scope andselect) objects.
</h2>

 May match selectors of replication controllers and services. More info:
 http://kubernetes.io/docs/user-guide/labels

### • `app`

> metadata > labels > ["app"] = "my-app"

## Name must be unique within a namespace.

 Is required when creating resources, although
 some resources may allow a client to request the generation of an appropriate name
 automatically. Name is primarily intended for creation idempotence and configuration
 definition. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/identifiers#names

> metadata > name = "my-app"

## Namespace defines the space within each name must be unique.

 An empty namespace is
 equivalent to the "default" namespace, but "default" is the canonical representation. Not
 all objects are required to be scoped to a namespace - the value of this field for those
 objects will be empty.

 Must be a DNS_LABEL. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/namespaces

> metadata > namespace = null

## List of objects depended by this object.

 If ALL objects in the list have been deleted,
 this object will be garbage collected. If this object is managed by a controller, then an
 entry in this list will point to this controller, with the controller field set to true.
 There cannot be more than one managing controller.

> metadata > ownerReferences = null

<h2>
An opaque value that represents the internal version of this object that can be used byclients to determine when objects have changed.
</h2>

 May be used for optimistic concurrency,
 change detection, and the watch operation on a resource or set of resources. Clients must
 treat these values as opaque and passed unmodified back to the server. They may only be
 valid for a particular resource or set of resources.

 Populated by the system. Read-only. Value must be treated as opaque by clients and . More
 info:
 http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#concurrency-control-and-consistency

> metadata > resourceVersion = null

## SelfLink is a URL representing this object. Populated by the system. Read-only.

> metadata > selfLink = null

## UID is the unique in time and space value for this object.

 It is typically generated by
 the server on successful creation of a resource and is not allowed to change on PUT
 operations.

 Populated by the system. Read-only. More info:
 http://kubernetes.io/docs/user-guide/identifiers#uids

> metadata > uid = null

# DeploymentSpec is the specification of the desired behavior of the Deployment.

<h2>
Minimum number of seconds for which a newly created pod should be ready without any ofits container crashing, for it to be considered available. Defaults to 0 (pod will beconsidered available as soon as it is ready)
</h2>

> spec > minReadySeconds = null

## Indicates that the deployment is paused.

> spec > paused = null

<h2>
The maximum time in seconds for a deployment to make progress before it is considered tobe failed.
</h2>

 The deployment controller will continue to process failed deployments and a
 condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment
 status.

 Once autoRollback is implemented, the deployment controller will automatically
 rollback failed deployments. Note that progress will not be estimated during the time a
 deployment is paused. Defaults to 600s.

> spec > progressDeadlineSeconds = null

## Number of desired pods.

 This is a pointer to distinguish between explicit zero and not
 specified. Defaults to 1.

> spec > replicas = 3

## The number of old ReplicaSets to retain to allow rollback.

 This is a pointer to
 distinguish between explicit zero and not specified.
 Defaults to 2.

> spec > revisionHistoryLimit = null

> spec > rollbackTo = null

## A label selector is a label query over a set of resources.

 The result of matchLabels and
 matchExpressions are ANDed. An empty label selector matches all objects. A null label
 selector matches no objects.

### matchExpressions is a list of label selector requirements.

 The requirements are ANDed.

> spec > selector > matchExpressions = null

### matchLabels is a map of {key,value} pairs.

 A single {key,value} in the matchLabels map is
 equivalent to an element of matchExpressions, whose key field is "key", the operator is
 "In", and the values array contains only "value". The requirements are ANDed.

#### • `app`

> spec > selector > matchLabels > ["app"] = "my-app"

## DeploymentStrategy describes how to replace existing pods with new ones.

> spec > strategy = null

## PodTemplateSpec describes the data a pod should have when created from a template

<h3>
ObjectMeta is metadata that all persisted resources must have, which includes all objectsusers must create.
</h3>

<h4>
Annotations is an unstructured key value map stored with a resource that may be set byexternal tools to store and retrieve arbitrary metadata.
</h4>

 They are not queryable and
 should be preserved when modifying objects. More info:
 http://kubernetes.io/docs/user-guide/annotations

> spec > template > metadata > annotations = null

#### The name of the cluster which the object belongs to.

 This is used to distinguish
 resources with same name and namespace in different clusters. This field is not set
 anywhere right now and apiserver is going to ignore it if set in create or update request.

> spec > template > metadata > clusterName = null

> spec > template > metadata > creationTimestamp = null

<h4>
Number of seconds allowed for this object to gracefully terminate before it will beremoved from the system.
</h4>

 Only set when deletionTimestamp is also set. May only be
 shortened. Read-only.

> spec > template > metadata > deletionGracePeriodSeconds = null

> spec > template > metadata > deletionTimestamp = null

#### Must be empty before the object is deleted from the registry.

 Each entry is an identifier
 for the responsible component that will remove the entry from the list. If the
 deletionTimestamp of the object is non-nil, entries in this list can only be removed.

> spec > template > metadata > finalizers = null

<h4>
GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IFthe Name field has not been provided.
</h4>

 If this field is used, the name returned to the
 client will be different than the name passed. This value will also be combined with a
 unique suffix. The provided value has the same validation rules as the Name field, and
 may be truncated by the length of the suffix required to make the value unique on the
 server.

 If this field is specified and the generated name exists, the server will NOT return a
 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout
 indicating a unique name could not be found in the time allotted, and the client should
 retry (optionally after the time indicated in the Retry-After header).

 Applied only if Name is not specified. More info:
 http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#idempotency

> spec > template > metadata > generateName = null

#### A sequence number representing a specific generation of the desired state.

 Populated by
 the system. Read-only.

> spec > template > metadata > generation = null

<h4>
Map of string keys and values that can be used to organize and categorize (scope andselect) objects.
</h4>

 May match selectors of replication controllers and services. More info:
 http://kubernetes.io/docs/user-guide/labels

##### • `app`

> spec > template > metadata > labels > ["app"] = "my-app"

#### Name must be unique within a namespace.

 Is required when creating resources, although
 some resources may allow a client to request the generation of an appropriate name
 automatically. Name is primarily intended for creation idempotence and configuration
 definition. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > metadata > name = null

#### Namespace defines the space within each name must be unique.

 An empty namespace is
 equivalent to the "default" namespace, but "default" is the canonical representation. Not
 all objects are required to be scoped to a namespace - the value of this field for those
 objects will be empty.

 Must be a DNS_LABEL. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/namespaces

> spec > template > metadata > namespace = null

#### List of objects depended by this object.

 If ALL objects in the list have been deleted,
 this object will be garbage collected. If this object is managed by a controller, then an
 entry in this list will point to this controller, with the controller field set to true.
 There cannot be more than one managing controller.

> spec > template > metadata > ownerReferences = null

<h4>
An opaque value that represents the internal version of this object that can be used byclients to determine when objects have changed.
</h4>

 May be used for optimistic concurrency,
 change detection, and the watch operation on a resource or set of resources. Clients must
 treat these values as opaque and passed unmodified back to the server. They may only be
 valid for a particular resource or set of resources.

 Populated by the system. Read-only. Value must be treated as opaque by clients and . More
 info:
 http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#concurrency-control-and-consistency

> spec > template > metadata > resourceVersion = null

#### SelfLink is a URL representing this object.

 Populated by the system. Read-only.

> spec > template > metadata > selfLink = null

#### UID is the unique in time and space value for this object.

 It is typically generated by
 the server on successful creation of a resource and is not allowed to change on PUT
 operations.

 Populated by the system.

 Read-only. More info:
 http://kubernetes.io/docs/user-guide/identifiers#uids

> spec > template > metadata > uid = null

### PodSpec is a description of a pod.

<h4>
Optional duration in seconds the pod may be active on the node relative to StartTimebefore the system will actively try to mark it failed and kill associated containers.
</h4>


 Value must be a positive integer.

> spec > template > spec > activeDeadlineSeconds = null

#### Affinity is a group of affinity scheduling rules.

> spec > template > spec > affinity = null

<h4>
AutomountServiceAccountToken indicates whether a service account token should beautomatically mounted.
</h4>

> spec > template > spec > automountServiceAccountToken = null

#### List of containers belonging to the pod.

 Containers cannot currently be added or removed.
 There must be at least one container in a Pod. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/containers

##### • `0`

###### Arguments to the entrypoint.

 The docker image's CMD is used if this is not provided.
 Variable references $(VAR_NAME) are expanded using the container's environment. If a
 variable cannot be resolved, the reference in the input string will be unchanged. The
 $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references
 will never be expanded, regardless of whether the variable exists or not. Cannot be
 updated. More info:
 http://kubernetes.io/docs/user-guide/containers#containers-and-commands

> spec > template > spec > containers > [0] > args = null

###### Entrypoint array.

 Not executed within a shell. The docker image's ENTRYPOINT is used if
 this is not provided. Variable references $(VAR_NAME) are expanded using the container's
 environment. If a variable cannot be resolved, the reference in the input string will be
 unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME).
 Escaped references will never be expanded, regardless of whether the variable exists or
 not. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/containers#containers-and-commands

> spec > template > spec > containers > [0] > command = null

###### List of environment variables to set in the container.

 Cannot be updated.

###### • `0`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [0] > env > [0] > name = "DB_HOST"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [0] > env > [0] > value = "db-service"

###### EnvVarSource represents a source for the value of an EnvVar.

> spec > template > spec > containers > [0] > env > [0] > valueFrom = null

###### • `1`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [0] > env > [1] > name = "DB_PORT"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [0] > env > [1] > value = "5432"

###### EnvVarSource represents a source for the value of an EnvVar.

> spec > template > spec > containers > [0] > env > [1] > valueFrom = null

###### • `2`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [0] > env > [2] > name = "DB_USER"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [0] > env > [2] > value = null

###### EnvVarSource represents a source for the value of an EnvVar.

###### Selects a key from a ConfigMap.

> spec > template > spec > containers > [0] > env > [2] > valueFrom > configMapKeyRef = null

###### ObjectFieldSelector selects an APIVersioned field of an object.

> spec > template > spec > containers > [0] > env > [2] > valueFrom > fieldRef = null

###### ResourceFieldSelector represents container resources (cpu, memory) and their output format

> spec > template > spec > containers > [0] > env > [2] > valueFrom > resourceFieldRef = null

###### SecretKeySelector selects a key of a Secret.

###### The key of the secret to select from.

 Must be a valid secret key.

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > key = "username"

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

###### Specify whether the Secret or it's key must be defined

> spec > template > spec > containers > [0] > env > [2] > valueFrom > secretKeyRef > optional = null

###### • `3`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [0] > env > [3] > name = "DB_PASSWORD"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [0] > env > [3] > value = null

###### EnvVarSource represents a source for the value of an EnvVar.

###### Selects a key from a ConfigMap.

> spec > template > spec > containers > [0] > env > [3] > valueFrom > configMapKeyRef = null

###### ObjectFieldSelector selects an APIVersioned field of an object.

> spec > template > spec > containers > [0] > env > [3] > valueFrom > fieldRef = null

###### ResourceFieldSelector represents container resources (cpu, memory) and their output format

> spec > template > spec > containers > [0] > env > [3] > valueFrom > resourceFieldRef = null

###### SecretKeySelector selects a key of a Secret.

###### The key of the secret to select from.

 Must be a valid secret key.

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > key = "password"

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

###### Specify whether the Secret or it's key must be defined

> spec > template > spec > containers > [0] > env > [3] > valueFrom > secretKeyRef > optional = null

###### List of sources to populate environment variables in the container.

 The keys defined
 within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event
 when the container is starting. When a key exists in multiple sources, the value
 associated with the last source will take precedence. Values defined by an Env with a
 duplicate key will take precedence. Cannot be updated.

> spec > template > spec > containers > [0] > envFrom = null

###### Docker image name.

 More info: http://kubernetes.io/docs/user-guide/images

> spec > template > spec > containers > [0] > image = "my-frontend-image:v1.0.0"

###### Image pull policy.

 One of Always, Never, IfNotPresent. Defaults to Always if :latest tag
 is specified, or IfNotPresent otherwise. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/images#updating-images

> spec > template > spec > containers > [0] > imagePullPolicy = null

<h6>
Lifecycle describes actions that the management system should take in response tocontainer lifecycle events.
</h6>

 For the PostStart and PreStop lifecycle handlers, management
 of the container blocks until the action is complete, unless the container process fails,
 in which case the handler is aborted.

> spec > template > spec > containers > [0] > lifecycle = null

<h6>
Probe describes a health check to be performed against a container to determine whetherit is alive or ready to receive traffic.
</h6>

> spec > template > spec > containers > [0] > livenessProbe = null

###### Name of the container specified as a DNS_LABEL.

 Each container in a pod must have a
 unique name (DNS_LABEL). Cannot be updated.

> spec > template > spec > containers > [0] > name = "frontend"

###### List of ports to expose from the container.

 Exposing a port here gives the system
 additional information about the network connections a container uses, but is primarily
 informational. Not specifying a port here DOES NOT prevent that port from being exposed.
 Any port which is listening on the default "0.0.0.0" address inside a container will be
 accessible from the network. Cannot be updated.

###### • `0`

###### Number of port to expose on the pod's IP address.

 This must be a valid port number, 0 < x
 < 65536.

> spec > template > spec > containers > [0] > ports > [0] > containerPort = 80

###### What host IP to bind the external port to.

> spec > template > spec > containers > [0] > ports > [0] > hostIP = null

###### Number of port to expose on the host.

 If specified, this must be a valid port number, 0 <
 x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do
 not need this.

> spec > template > spec > containers > [0] > ports > [0] > hostPort = null

###### If specified, this must be an IANA_SVC_NAME and unique within the pod.

 Each named port in
 a pod must have a unique name. Name for the port that can be referred to by services.

> spec > template > spec > containers > [0] > ports > [0] > name = null

###### Protocol for port. Must be UDP or TCP. Defaults to "TCP".

> spec > template > spec > containers > [0] > ports > [0] > protocol = null

<h6>
Probe describes a health check to be performed against a container to determine whetherit is alive or ready to receive traffic.
</h6>

> spec > template > spec > containers > [0] > readinessProbe = null

###### ResourceRequirements describes the compute resource requirements.

> spec > template > spec > containers > [0] > resources = null

<h6>
SecurityContext holds security configuration that will be applied to a container. Somefields are present in both SecurityContext and PodSecurityContext.
</h6>

 When both are set,
 the values in SecurityContext take precedence.

> spec > template > spec > containers > [0] > securityContext = null

<h6>
Whether this container should allocate a buffer for stdin in the container runtime. Ifthis is not set, reads from stdin in the container will always result in EOF. Default isfalse.
</h6>

> spec > template > spec > containers > [0] > stdin = null

<h6>
Whether the container runtime should close the stdin channel after it has been opened bya single attach.
</h6>

 When stdin is true the stdin stream will remain open across multiple
 attach sessions. If stdinOnce is set to true, stdin is opened on container start, is
 empty until the first client attaches to stdin, and then remains open and accepts data
 until the client disconnects, at which time stdin is closed and remains closed until the
 container is restarted. If this flag is false, a container processes that reads from
 stdin will never receive an EOF. Default is false

> spec > template > spec > containers > [0] > stdinOnce = null

<h6>
Optional: Path at which the file to which the container's termination message will bewritten is mounted into the container's filesystem.
</h6>

 Message written is intended to be
 brief final status, such as an assertion failure message. Will be truncated by the node
 if greater than 4096 bytes. The total message length across all containers will be
 limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.

> spec > template > spec > containers > [0] > terminationMessagePath = null

###### Indicate how the termination message should be populated.

 File will use the contents of
 terminationMessagePath to populate the container status message on both success and
 failure. FallbackToLogsOnError will use the last chunk of container log output if the
 termination message file is empty and the container exited with an error. The log output
 is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be
 updated.

> spec > template > spec > containers > [0] > terminationMessagePolicy = null

<h6>
Whether this container should allocate a TTY for itself, also requires 'stdin' to betrue.
</h6>

 Default is false.

> spec > template > spec > containers > [0] > tty = null

###### Pod volumes to mount into the container's filesystem.

 Cannot be updated.

###### • `0`

###### Path within the container at which the volume should be mounted.

 Must not contain ':'.

> spec > template > spec > containers > [0] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

###### This must match the Name of a Volume.

> spec > template > spec > containers > [0] > volumeMounts > [0] > name = "frontend-config"

###### Mounted read-only if true, read-write otherwise (false or unspecified).

 Defaults to false.

> spec > template > spec > containers > [0] > volumeMounts > [0] > readOnly = null

###### Path within the volume from which the container's volume should be mounted.

 Defaults to
 "" (volume's root).

> spec > template > spec > containers > [0] > volumeMounts > [0] > subPath = null

###### • `1`

###### Path within the container at which the volume should be mounted.

 Must not contain ':'.

> spec > template > spec > containers > [0] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

###### This must match the Name of a Volume.

> spec > template > spec > containers > [0] > volumeMounts > [1] > name = "shared-data"

###### Mounted read-only if true, read-write otherwise (false or unspecified).

 Defaults to false.

> spec > template > spec > containers > [0] > volumeMounts > [1] > readOnly = null

###### Path within the volume from which the container's volume should be mounted.

 Defaults to
 "" (volume's root).

> spec > template > spec > containers > [0] > volumeMounts > [1] > subPath = null

###### Container's working directory.

 If not specified, the container runtime's default will be
 used, which might be configured in the container image. Cannot be updated.

> spec > template > spec > containers > [0] > workingDir = null

##### • `1`

###### Arguments to the entrypoint.

 The docker image's CMD is used if this is not provided.
 Variable references $(VAR_NAME) are expanded using the container's environment. If a
 variable cannot be resolved, the reference in the input string will be unchanged. The
 $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references
 will never be expanded, regardless of whether the variable exists or not. Cannot be
 updated. More info:
 http://kubernetes.io/docs/user-guide/containers#containers-and-commands

> spec > template > spec > containers > [1] > args = null

###### Entrypoint array.

 Not executed within a shell. The docker image's ENTRYPOINT is used if
 this is not provided. Variable references $(VAR_NAME) are expanded using the container's
 environment. If a variable cannot be resolved, the reference in the input string will be
 unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME).
 Escaped references will never be expanded, regardless of whether the variable exists or
 not. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/containers#containers-and-commands

> spec > template > spec > containers > [1] > command = null

###### List of environment variables to set in the container.

 Cannot be updated.

###### • `0`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [1] > env > [0] > name = "DB_HOST"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [1] > env > [0] > value = "db-service"

###### EnvVarSource represents a source for the value of an EnvVar.

> spec > template > spec > containers > [1] > env > [0] > valueFrom = null

###### • `1`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [1] > env > [1] > name = "DB_PORT"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [1] > env > [1] > value = "5432"

###### EnvVarSource represents a source for the value of an EnvVar.

> spec > template > spec > containers > [1] > env > [1] > valueFrom = null

###### • `2`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [1] > env > [2] > name = "DB_USER"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [1] > env > [2] > value = null

###### EnvVarSource represents a source for the value of an EnvVar.

###### Selects a key from a ConfigMap.

> spec > template > spec > containers > [1] > env > [2] > valueFrom > configMapKeyRef = null

###### ObjectFieldSelector selects an APIVersioned field of an object.

> spec > template > spec > containers > [1] > env > [2] > valueFrom > fieldRef = null

###### ResourceFieldSelector represents container resources (cpu, memory) and their output format

> spec > template > spec > containers > [1] > env > [2] > valueFrom > resourceFieldRef = null

###### SecretKeySelector selects a key of a Secret.

###### The key of the secret to select from.

 Must be a valid secret key.

> spec > template > spec > containers > [1] > env > [2] > valueFrom > secretKeyRef > key = "username"

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > containers > [1] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

###### Specify whether the Secret or it's key must be defined

> spec > template > spec > containers > [1] > env > [2] > valueFrom > secretKeyRef > optional = null

###### • `3`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [1] > env > [3] > name = "DB_PASSWORD"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [1] > env > [3] > value = null

###### EnvVarSource represents a source for the value of an EnvVar.

###### Selects a key from a ConfigMap.

> spec > template > spec > containers > [1] > env > [3] > valueFrom > configMapKeyRef = null

###### ObjectFieldSelector selects an APIVersioned field of an object.

> spec > template > spec > containers > [1] > env > [3] > valueFrom > fieldRef = null

###### ResourceFieldSelector represents container resources (cpu, memory) and their output format

> spec > template > spec > containers > [1] > env > [3] > valueFrom > resourceFieldRef = null

###### SecretKeySelector selects a key of a Secret.

###### The key of the secret to select from.

 Must be a valid secret key.

> spec > template > spec > containers > [1] > env > [3] > valueFrom > secretKeyRef > key = "password"

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > containers > [1] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

###### Specify whether the Secret or it's key must be defined

> spec > template > spec > containers > [1] > env > [3] > valueFrom > secretKeyRef > optional = null

###### List of sources to populate environment variables in the container.

 The keys defined
 within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event
 when the container is starting. When a key exists in multiple sources, the value
 associated with the last source will take precedence. Values defined by an Env with a
 duplicate key will take precedence. Cannot be updated.

> spec > template > spec > containers > [1] > envFrom = null

###### Docker image name.

 More info: http://kubernetes.io/docs/user-guide/images

> spec > template > spec > containers > [1] > image = "my-backend-image:v1.0.0"

###### Image pull policy.

 One of Always, Never, IfNotPresent. Defaults to Always if :latest tag
 is specified, or IfNotPresent otherwise. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/images#updating-images

> spec > template > spec > containers > [1] > imagePullPolicy = null

<h6>
Lifecycle describes actions that the management system should take in response tocontainer lifecycle events.
</h6>

 For the PostStart and PreStop lifecycle handlers, management
 of the container blocks until the action is complete, unless the container process fails,
 in which case the handler is aborted.

> spec > template > spec > containers > [1] > lifecycle = null

<h6>
Probe describes a health check to be performed against a container to determine whetherit is alive or ready to receive traffic.
</h6>

> spec > template > spec > containers > [1] > livenessProbe = null

###### Name of the container specified as a DNS_LABEL.

 Each container in a pod must have a
 unique name (DNS_LABEL). Cannot be updated.

> spec > template > spec > containers > [1] > name = "backend"

###### List of ports to expose from the container.

 Exposing a port here gives the system
 additional information about the network connections a container uses, but is primarily
 informational. Not specifying a port here DOES NOT prevent that port from being exposed.
 Any port which is listening on the default "0.0.0.0" address inside a container will be
 accessible from the network. Cannot be updated.

###### • `0`

###### Number of port to expose on the pod's IP address.

 This must be a valid port number, 0 < x
 < 65536.

> spec > template > spec > containers > [1] > ports > [0] > containerPort = 8080

###### What host IP to bind the external port to.

> spec > template > spec > containers > [1] > ports > [0] > hostIP = null

###### Number of port to expose on the host.

 If specified, this must be a valid port number, 0 <
 x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do
 not need this.

> spec > template > spec > containers > [1] > ports > [0] > hostPort = null

###### If specified, this must be an IANA_SVC_NAME and unique within the pod.

 Each named port in
 a pod must have a unique name. Name for the port that can be referred to by services.

> spec > template > spec > containers > [1] > ports > [0] > name = null

###### Protocol for port. Must be UDP or TCP. Defaults to "TCP".

> spec > template > spec > containers > [1] > ports > [0] > protocol = null

<h6>
Probe describes a health check to be performed against a container to determine whetherit is alive or ready to receive traffic.
</h6>

> spec > template > spec > containers > [1] > readinessProbe = null

###### ResourceRequirements describes the compute resource requirements.

> spec > template > spec > containers > [1] > resources = null

<h6>
SecurityContext holds security configuration that will be applied to a container. Somefields are present in both SecurityContext and PodSecurityContext.
</h6>

 When both are set,
 the values in SecurityContext take precedence.

> spec > template > spec > containers > [1] > securityContext = null

<h6>
Whether this container should allocate a buffer for stdin in the container runtime. Ifthis is not set, reads from stdin in the container will always result in EOF. Default isfalse.
</h6>

> spec > template > spec > containers > [1] > stdin = null

<h6>
Whether the container runtime should close the stdin channel after it has been opened bya single attach.
</h6>

 When stdin is true the stdin stream will remain open across multiple
 attach sessions. If stdinOnce is set to true, stdin is opened on container start, is
 empty until the first client attaches to stdin, and then remains open and accepts data
 until the client disconnects, at which time stdin is closed and remains closed until the
 container is restarted. If this flag is false, a container processes that reads from
 stdin will never receive an EOF. Default is false

> spec > template > spec > containers > [1] > stdinOnce = null

<h6>
Optional: Path at which the file to which the container's termination message will bewritten is mounted into the container's filesystem.
</h6>

 Message written is intended to be
 brief final status, such as an assertion failure message. Will be truncated by the node
 if greater than 4096 bytes. The total message length across all containers will be
 limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.

> spec > template > spec > containers > [1] > terminationMessagePath = null

###### Indicate how the termination message should be populated.

 File will use the contents of
 terminationMessagePath to populate the container status message on both success and
 failure. FallbackToLogsOnError will use the last chunk of container log output if the
 termination message file is empty and the container exited with an error. The log output
 is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be
 updated.

> spec > template > spec > containers > [1] > terminationMessagePolicy = null

<h6>
Whether this container should allocate a TTY for itself, also requires 'stdin' to betrue.
</h6>

 Default is false.

> spec > template > spec > containers > [1] > tty = null

###### Pod volumes to mount into the container's filesystem.

 Cannot be updated.

###### • `0`

###### Path within the container at which the volume should be mounted.

 Must not contain ':'.

> spec > template > spec > containers > [1] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

###### This must match the Name of a Volume.

> spec > template > spec > containers > [1] > volumeMounts > [0] > name = "backend-config"

###### Mounted read-only if true, read-write otherwise (false or unspecified).

 Defaults to false.

> spec > template > spec > containers > [1] > volumeMounts > [0] > readOnly = null

###### Path within the volume from which the container's volume should be mounted.

 Defaults to
 "" (volume's root).

> spec > template > spec > containers > [1] > volumeMounts > [0] > subPath = null

###### • `1`

###### Path within the container at which the volume should be mounted.

 Must not contain ':'.

> spec > template > spec > containers > [1] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

###### This must match the Name of a Volume.

> spec > template > spec > containers > [1] > volumeMounts > [1] > name = "shared-data"

###### Mounted read-only if true, read-write otherwise (false or unspecified).

 Defaults to false.

> spec > template > spec > containers > [1] > volumeMounts > [1] > readOnly = null

###### Path within the volume from which the container's volume should be mounted.

 Defaults to
 "" (volume's root).

> spec > template > spec > containers > [1] > volumeMounts > [1] > subPath = null

###### Container's working directory.

 If not specified, the container runtime's default will be
 used, which might be configured in the container image. Cannot be updated.

> spec > template > spec > containers > [1] > workingDir = null

##### • `2`

###### Arguments to the entrypoint.

 The docker image's CMD is used if this is not provided.
 Variable references $(VAR_NAME) are expanded using the container's environment. If a
 variable cannot be resolved, the reference in the input string will be unchanged. The
 $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references
 will never be expanded, regardless of whether the variable exists or not. Cannot be
 updated. More info:
 http://kubernetes.io/docs/user-guide/containers#containers-and-commands

> spec > template > spec > containers > [2] > args = null

###### Entrypoint array.

 Not executed within a shell. The docker image's ENTRYPOINT is used if
 this is not provided. Variable references $(VAR_NAME) are expanded using the container's
 environment. If a variable cannot be resolved, the reference in the input string will be
 unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME).
 Escaped references will never be expanded, regardless of whether the variable exists or
 not. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/containers#containers-and-commands

> spec > template > spec > containers > [2] > command = null

###### List of environment variables to set in the container.

 Cannot be updated.

###### • `0`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [2] > env > [0] > name = "DB_HOST"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [2] > env > [0] > value = "db-service"

###### EnvVarSource represents a source for the value of an EnvVar.

> spec > template > spec > containers > [2] > env > [0] > valueFrom = null

###### • `1`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [2] > env > [1] > name = "DB_PORT"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [2] > env > [1] > value = "5432"

###### EnvVarSource represents a source for the value of an EnvVar.

> spec > template > spec > containers > [2] > env > [1] > valueFrom = null

###### • `2`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [2] > env > [2] > name = "DB_USER"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [2] > env > [2] > value = null

###### EnvVarSource represents a source for the value of an EnvVar.

###### Selects a key from a ConfigMap.

> spec > template > spec > containers > [2] > env > [2] > valueFrom > configMapKeyRef = null

###### ObjectFieldSelector selects an APIVersioned field of an object.

> spec > template > spec > containers > [2] > env > [2] > valueFrom > fieldRef = null

###### ResourceFieldSelector represents container resources (cpu, memory) and their output format

> spec > template > spec > containers > [2] > env > [2] > valueFrom > resourceFieldRef = null

###### SecretKeySelector selects a key of a Secret.

###### The key of the secret to select from.

 Must be a valid secret key.

> spec > template > spec > containers > [2] > env > [2] > valueFrom > secretKeyRef > key = "username"

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > containers > [2] > env > [2] > valueFrom > secretKeyRef > name = "db-credentials"

###### Specify whether the Secret or it's key must be defined

> spec > template > spec > containers > [2] > env > [2] > valueFrom > secretKeyRef > optional = null

###### • `3`

###### Name of the environment variable.

 Must be a C_IDENTIFIER.

> spec > template > spec > containers > [2] > env > [3] > name = "DB_PASSWORD"

<h6>
Variable references $(VAR_NAME) are expanded using the previous defined environmentvariables in the container and any service environment variables.
</h6>

 If a variable cannot be
 resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
 be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
 regardless of whether the variable exists or not. Defaults to "".

> spec > template > spec > containers > [2] > env > [3] > value = null

###### EnvVarSource represents a source for the value of an EnvVar.

###### Selects a key from a ConfigMap.

> spec > template > spec > containers > [2] > env > [3] > valueFrom > configMapKeyRef = null

###### ObjectFieldSelector selects an APIVersioned field of an object.

> spec > template > spec > containers > [2] > env > [3] > valueFrom > fieldRef = null

###### ResourceFieldSelector represents container resources (cpu, memory) and their output format

> spec > template > spec > containers > [2] > env > [3] > valueFrom > resourceFieldRef = null

###### SecretKeySelector selects a key of a Secret.

###### The key of the secret to select from.

 Must be a valid secret key.

> spec > template > spec > containers > [2] > env > [3] > valueFrom > secretKeyRef > key = "password"

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > containers > [2] > env > [3] > valueFrom > secretKeyRef > name = "db-credentials"

###### Specify whether the Secret or it's key must be defined

> spec > template > spec > containers > [2] > env > [3] > valueFrom > secretKeyRef > optional = null

###### List of sources to populate environment variables in the container.

 The keys defined
 within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event
 when the container is starting. When a key exists in multiple sources, the value
 associated with the last source will take precedence. Values defined by an Env with a
 duplicate key will take precedence. Cannot be updated.

> spec > template > spec > containers > [2] > envFrom = null

###### Docker image name.

 More info: http://kubernetes.io/docs/user-guide/images

> spec > template > spec > containers > [2] > image = "my-job-worker-image:v1.0.0"

###### Image pull policy.

 One of Always, Never, IfNotPresent. Defaults to Always if :latest tag
 is specified, or IfNotPresent otherwise. Cannot be updated. More info:
 http://kubernetes.io/docs/user-guide/images#updating-images

> spec > template > spec > containers > [2] > imagePullPolicy = null

<h6>
Lifecycle describes actions that the management system should take in response tocontainer lifecycle events.
</h6>

 For the PostStart and PreStop lifecycle handlers, management
 of the container blocks until the action is complete, unless the container process fails,
 in which case the handler is aborted.

> spec > template > spec > containers > [2] > lifecycle = null

<h6>
Probe describes a health check to be performed against a container to determine whetherit is alive or ready to receive traffic.
</h6>

> spec > template > spec > containers > [2] > livenessProbe = null

###### Name of the container specified as a DNS_LABEL.

 Each container in a pod must have a
 unique name (DNS_LABEL). Cannot be updated.

> spec > template > spec > containers > [2] > name = "job-worker"

###### List of ports to expose from the container.

 Exposing a port here gives the system
 additional information about the network connections a container uses, but is primarily
 informational. Not specifying a port here DOES NOT prevent that port from being exposed.
 Any port which is listening on the default "0.0.0.0" address inside a container will be
 accessible from the network. Cannot be updated.

> spec > template > spec > containers > [2] > ports = null

<h6>
Probe describes a health check to be performed against a container to determine whetherit is alive or ready to receive traffic.
</h6>

> spec > template > spec > containers > [2] > readinessProbe = null

###### ResourceRequirements describes the compute resource requirements.

> spec > template > spec > containers > [2] > resources = null

<h6>
SecurityContext holds security configuration that will be applied to a container. Somefields are present in both SecurityContext and PodSecurityContext.
</h6>

 When both are set,
 the values in SecurityContext take precedence.

> spec > template > spec > containers > [2] > securityContext = null

<h6>
Whether this container should allocate a buffer for stdin in the container runtime. Ifthis is not set, reads from stdin in the container will always result in EOF. Default isfalse.
</h6>

> spec > template > spec > containers > [2] > stdin = null

<h6>
Whether the container runtime should close the stdin channel after it has been opened bya single attach.
</h6>

 When stdin is true the stdin stream will remain open across multiple
 attach sessions. If stdinOnce is set to true, stdin is opened on container start, is
 empty until the first client attaches to stdin, and then remains open and accepts data
 until the client disconnects, at which time stdin is closed and remains closed until the
 container is restarted. If this flag is false, a container processes that reads from
 stdin will never receive an EOF. Default is false

> spec > template > spec > containers > [2] > stdinOnce = null

<h6>
Optional: Path at which the file to which the container's termination message will bewritten is mounted into the container's filesystem.
</h6>

 Message written is intended to be
 brief final status, such as an assertion failure message. Will be truncated by the node
 if greater than 4096 bytes. The total message length across all containers will be
 limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.

> spec > template > spec > containers > [2] > terminationMessagePath = null

###### Indicate how the termination message should be populated.

 File will use the contents of
 terminationMessagePath to populate the container status message on both success and
 failure. FallbackToLogsOnError will use the last chunk of container log output if the
 termination message file is empty and the container exited with an error. The log output
 is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be
 updated.

> spec > template > spec > containers > [2] > terminationMessagePolicy = null

<h6>
Whether this container should allocate a TTY for itself, also requires 'stdin' to betrue.
</h6>

 Default is false.

> spec > template > spec > containers > [2] > tty = null

###### Pod volumes to mount into the container's filesystem.

 Cannot be updated.

###### • `0`

###### Path within the container at which the volume should be mounted.

 Must not contain ':'.

> spec > template > spec > containers > [2] > volumeMounts > [0] > mountPath = "/usr/src/app/config"

###### This must match the Name of a Volume.

> spec > template > spec > containers > [2] > volumeMounts > [0] > name = "job-worker-config"

###### Mounted read-only if true, read-write otherwise (false or unspecified).

 Defaults to false.

> spec > template > spec > containers > [2] > volumeMounts > [0] > readOnly = null

###### Path within the volume from which the container's volume should be mounted.

 Defaults to
 "" (volume's root).

> spec > template > spec > containers > [2] > volumeMounts > [0] > subPath = null

###### • `1`

###### Path within the container at which the volume should be mounted.

 Must not contain ':'.

> spec > template > spec > containers > [2] > volumeMounts > [1] > mountPath = "/usr/src/app/data"

###### This must match the Name of a Volume.

> spec > template > spec > containers > [2] > volumeMounts > [1] > name = "shared-data"

###### Mounted read-only if true, read-write otherwise (false or unspecified).

 Defaults to false.

> spec > template > spec > containers > [2] > volumeMounts > [1] > readOnly = null

###### Path within the volume from which the container's volume should be mounted.

 Defaults to
 "" (volume's root).

> spec > template > spec > containers > [2] > volumeMounts > [1] > subPath = null

###### Container's working directory.

 If not specified, the container runtime's default will be
 used, which might be configured in the container image. Cannot be updated.

> spec > template > spec > containers > [2] > workingDir = null

#### Set DNS policy for containers within the pod.

 One of 'ClusterFirstWithHostNet',
 'ClusterFirst' or 'Default'. Defaults to "ClusterFirst". To have DNS options set along
 with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.

> spec > template > spec > dnsPolicy = null

#### Use the host's ipc namespace.

 Optional: Default to false.

> spec > template > spec > hostIPC = null

<h4>
Specifies the hostname of the Pod If not specified, the pod's hostname will be set to asystem-defined value.
</h4>

> spec > template > spec > hostname = null

#### Host networking requested for this pod.

 Use the host's network namespace. If this option
 is set, the ports that will be used must be specified. Default to false.

> spec > template > spec > hostNetwork = null

#### Use the host's pid namespace. Optional: Default to false.

> spec > template > spec > hostPID = null

<h4>
ImagePullSecrets is an optional list of references to secrets in the same namespace touse for pulling any of the images used by this PodSpec.
</h4>

 If specified, these secrets will
 be passed to individual puller implementations for them to use. For example, in the case
 of docker, only DockerConfig type secrets are honored. More info:
 http://kubernetes.io/docs/user-guide/images#specifying-imagepullsecrets-on-a-pod

> spec > template > spec > imagePullSecrets = null

#### List of initialization containers belonging to the pod.

 Init containers are executed in
 order prior to containers being started. If any init container fails, the pod is
 considered to have failed and is handled according to its restartPolicy. The name for an
 init container or normal container must be unique among all containers. Init containers
 may not have Lifecycle actions, Readiness probes, or Liveness probes. The
 resourceRequirements of an init container are taken into account during scheduling by
 finding the highest request/limit for each resource type, and then using the max of of
 that value or the sum of the normal containers. Limits are applied to init containers in
 a similar fashion. Init containers cannot currently be added or removed. Cannot be
 updated. More info: http://kubernetes.io/docs/user-guide/containers

> spec > template > spec > initContainers = null

#### NodeName is a request to schedule this pod onto a specific node.

 If it is non-empty, the
 scheduler simply schedules this pod onto that node, assuming that it fits resource
 requirements.

> spec > template > spec > nodeName = null

#### NodeSelector is a selector which must be true for the pod to fit on a node.

 Selector
 which must match a node's labels for the pod to be scheduled on that node. More info:
 http://kubernetes.io/docs/user-guide/node-selection/README

> spec > template > spec > nodeSelector = null

#### Restart policy for all containers within the pod.

 One of Always, OnFailure, Never.
 Default to Always. More info:
 http://kubernetes.io/docs/user-guide/pod-states#restartpolicy

> spec > template > spec > restartPolicy = null

#### If specified, the pod will be dispatched by specified scheduler.

 If not specified, the
 pod will be dispatched by default scheduler.

> spec > template > spec > schedulerName = null

<h4>
PodSecurityContext holds pod-level security attributes and common container settings.Some fields are also present in container.securityContext.
</h4>

 Field values of
 container.securityContext take precedence over field values of PodSecurityContext.

> spec > template > spec > securityContext = null

#### DeprecatedServiceAccount is a depreciated alias for ServiceAccountName.

 Deprecated: Use
 serviceAccountName instead.

> spec > template > spec > serviceAccount = null

#### ServiceAccountName is the name of the ServiceAccount to use to run this pod.

 More info:
 http://releases.k8s.io/HEAD/docs/design/service_accounts.md

> spec > template > spec > serviceAccountName = null

<h4>
If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>.<podnamespace>.svc.<cluster domain>".
</h4>

 If not specified, the pod will not have a domainname at
 all.

> spec > template > spec > subdomain = null

#### Optional duration in seconds the pod needs to terminate gracefully.

 May be decreased in
 delete request. Value must be non-negative integer. The value zero indicates delete
 immediately. If this value is nil, the default grace period will be used instead. The
 grace period is the duration in seconds after the processes running in the pod are sent a
 termination signal and the time when the processes are forcibly halted with a kill
 signal. Set this value longer than the expected cleanup time for your process. Defaults
 to 30 seconds.

> spec > template > spec > terminationGracePeriodSeconds = null

#### If specified, the pod's tolerations.

> spec > template > spec > tolerations = null

#### List of volumes that can be mounted by containers belonging to the pod.

 More info:
 http://kubernetes.io/docs/user-guide/volumes

##### • `0`

###### Represents a Persistent Disk resource in AWS.

 An AWS EBS disk must exist before mounting to a container.

 The disk must also be in the
 same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS
 EBS volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [0] > awsElasticBlockStore = null

###### AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.

> spec > template > spec > volumes > [0] > azureDisk = null

###### AzureFile represents an Azure File Service mount on the host and bind mount to the pod.

> spec > template > spec > volumes > [0] > azureFile = null

<h6>
Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do notsupport ownership management or SELinux relabeling.
</h6>

> spec > template > spec > volumes > [0] > cephfs = null

###### Represents a cinder volume resource in Openstack.

 A Cinder volume must exist before
 mounting to a container. The volume must also be in the same region as the kubelet.
 Cinder volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [0] > cinder = null

###### Adapts a ConfigMap into a volume.

 The contents of the target ConfigMap's Data field will be presented in a volume as files
 using the keys in the Data field as the file names, unless the items element is populated
 with specific mappings of keys to paths.

 ConfigMap volumes support ownership management
 and SELinux relabeling.

###### Optional: mode bits to use on created files by default.

 Must be a value between 0 and
 0777. Defaults to 0644. Directories within the path are not affected by this setting.
 This might be in conflict with other options that affect the file mode, like fsGroup, and
 the result can be other mode bits set.

> spec > template > spec > volumes > [0] > configMap > defaultMode = null

<h6>
If unspecified, each key-value pair in the Data field of the referenced ConfigMap will beprojected into the volume as a file whose name is the key and content is the value.
</h6>

 If
 specified, the listed keys will be projected into the specified paths, and unlisted keys
 will not be present. If a key is specified which is not present in the ConfigMap, the
 volume setup will error unless it is marked optional. Paths must be relative and may not
 contain the '..' path or start with '..'.

> spec > template > spec > volumes > [0] > configMap > items = null

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > volumes > [0] > configMap > name = "frontend-config"

###### Specify whether the ConfigMap or it's keys must be defined

> spec > template > spec > volumes > [0] > configMap > optional = null

###### DownwardAPIVolumeSource represents a volume containing downward API info.

 Downward API
 volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [0] > downwardAPI = null

###### Represents an empty directory for a pod.

 Empty directory volumes support ownership
 management and SELinux relabeling.

> spec > template > spec > volumes > [0] > emptyDir = null

###### Represents a Fibre Channel volume.

 Fibre Channel volumes can only be mounted as
 read/write once. Fibre Channel volumes support ownership management and SELinux
 relabeling.

> spec > template > spec > volumes > [0] > fc = null

<h6>
FlexVolume represents a generic volume resource that is provisioned/attached using anexec based plugin.
</h6>

 This is an alpha feature and may change in future.

> spec > template > spec > volumes > [0] > flexVolume = null

###### Represents a Flocker volume mounted by the Flocker agent.

 One and only one of datasetName
 and datasetUUID should be set. Flocker volumes do not support ownership management or
 SELinux relabeling.

> spec > template > spec > volumes > [0] > flocker = null

###### Represents a Persistent Disk resource in Google Compute Engine.

 A GCE PD must exist before mounting to a container. The disk must also be in the same GCE
 project and zone as the kubelet. A GCE PD can only be mounted as read/write once or
 read-only many times. GCE PDs support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [0] > gcePersistentDisk = null

<h6>
Represents a volume that is populated with the contents of a git repository. Git repovolumes do not support ownership management.
</h6>

 Git repo volumes support SELinux relabeling.

> spec > template > spec > volumes > [0] > gitRepo = null

###### Represents a Glusterfs mount that lasts the lifetime of a pod.

 Glusterfs volumes do not
 support ownership management or SELinux relabeling.

> spec > template > spec > volumes > [0] > glusterfs = null

###### Represents a host path mapped into a pod.

 Host path volumes do not support ownership
 management or SELinux relabeling.

> spec > template > spec > volumes > [0] > hostPath = null

<h6>
Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSIvolumes support ownership management and SELinux relabeling.
</h6>

> spec > template > spec > volumes > [0] > iscsi = null

###### Volume's name.

 Must be a DNS_LABEL and unique within the pod. More info:
 http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > volumes > [0] > name = "frontend-config"

###### Represents an NFS mount that lasts the lifetime of a pod.

 NFS volumes do not support
 ownership management or SELinux relabeling.

> spec > template > spec > volumes > [0] > nfs = null

###### PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace.

 This
 volume finds the bound PV and mounts that volume for the pod. A
 PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of
 volume that is owned by someone else (the system).

> spec > template > spec > volumes > [0] > persistentVolumeClaim = null

###### Represents a Photon Controller persistent disk resource.

> spec > template > spec > volumes > [0] > photonPersistentDisk = null

###### PortworxVolumeSource represents a Portworx volume resource.

> spec > template > spec > volumes > [0] > portworxVolume = null

###### Represents a projected volume source

> spec > template > spec > volumes > [0] > projected = null

###### Represents a Quobyte mount that lasts the lifetime of a pod.

 Quobyte volumes do not
 support ownership management or SELinux relabeling.

> spec > template > spec > volumes > [0] > quobyte = null

###### Represents a Rados Block Device mount that lasts the lifetime of a pod.

 RBD volumes
 support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [0] > rbd = null

###### ScaleIOVolumeSource represents a persistent ScaleIO volume

> spec > template > spec > volumes > [0] > scaleIO = null

###### Adapts a Secret into a volume.

 The contents of the target Secret's Data field will be presented in a volume as files
 using the keys in the Data field as the file names. Secret volumes support ownership
 management and SELinux relabeling.

> spec > template > spec > volumes > [0] > secret = null

###### Represents a vSphere volume resource.

> spec > template > spec > volumes > [0] > vsphereVolume = null

##### • `1`

###### Represents a Persistent Disk resource in AWS.

 An AWS EBS disk must exist before mounting to a container.

 The disk must also be in the
 same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS
 EBS volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [1] > awsElasticBlockStore = null

###### AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.

> spec > template > spec > volumes > [1] > azureDisk = null

###### AzureFile represents an Azure File Service mount on the host and bind mount to the pod.

> spec > template > spec > volumes > [1] > azureFile = null

<h6>
Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do notsupport ownership management or SELinux relabeling.
</h6>

> spec > template > spec > volumes > [1] > cephfs = null

###### Represents a cinder volume resource in Openstack.

 A Cinder volume must exist before
 mounting to a container. The volume must also be in the same region as the kubelet.
 Cinder volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [1] > cinder = null

###### Adapts a ConfigMap into a volume.

 The contents of the target ConfigMap's Data field will be presented in a volume as files
 using the keys in the Data field as the file names, unless the items element is populated
 with specific mappings of keys to paths.

 ConfigMap volumes support ownership management
 and SELinux relabeling.

###### Optional: mode bits to use on created files by default.

 Must be a value between 0 and
 0777. Defaults to 0644. Directories within the path are not affected by this setting.
 This might be in conflict with other options that affect the file mode, like fsGroup, and
 the result can be other mode bits set.

> spec > template > spec > volumes > [1] > configMap > defaultMode = null

<h6>
If unspecified, each key-value pair in the Data field of the referenced ConfigMap will beprojected into the volume as a file whose name is the key and content is the value.
</h6>

 If
 specified, the listed keys will be projected into the specified paths, and unlisted keys
 will not be present. If a key is specified which is not present in the ConfigMap, the
 volume setup will error unless it is marked optional. Paths must be relative and may not
 contain the '..' path or start with '..'.

> spec > template > spec > volumes > [1] > configMap > items = null

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > volumes > [1] > configMap > name = "backend-config"

###### Specify whether the ConfigMap or it's keys must be defined

> spec > template > spec > volumes > [1] > configMap > optional = null

###### DownwardAPIVolumeSource represents a volume containing downward API info.

 Downward API
 volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [1] > downwardAPI = null

###### Represents an empty directory for a pod.

 Empty directory volumes support ownership
 management and SELinux relabeling.

> spec > template > spec > volumes > [1] > emptyDir = null

###### Represents a Fibre Channel volume.

 Fibre Channel volumes can only be mounted as
 read/write once. Fibre Channel volumes support ownership management and SELinux
 relabeling.

> spec > template > spec > volumes > [1] > fc = null

<h6>
FlexVolume represents a generic volume resource that is provisioned/attached using anexec based plugin.
</h6>

 This is an alpha feature and may change in future.

> spec > template > spec > volumes > [1] > flexVolume = null

###### Represents a Flocker volume mounted by the Flocker agent.

 One and only one of datasetName
 and datasetUUID should be set. Flocker volumes do not support ownership management or
 SELinux relabeling.

> spec > template > spec > volumes > [1] > flocker = null

###### Represents a Persistent Disk resource in Google Compute Engine.

 A GCE PD must exist before mounting to a container. The disk must also be in the same GCE
 project and zone as the kubelet. A GCE PD can only be mounted as read/write once or
 read-only many times. GCE PDs support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [1] > gcePersistentDisk = null

<h6>
Represents a volume that is populated with the contents of a git repository. Git repovolumes do not support ownership management.
</h6>

 Git repo volumes support SELinux relabeling.

> spec > template > spec > volumes > [1] > gitRepo = null

###### Represents a Glusterfs mount that lasts the lifetime of a pod.

 Glusterfs volumes do not
 support ownership management or SELinux relabeling.

> spec > template > spec > volumes > [1] > glusterfs = null

###### Represents a host path mapped into a pod.

 Host path volumes do not support ownership
 management or SELinux relabeling.

> spec > template > spec > volumes > [1] > hostPath = null

<h6>
Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSIvolumes support ownership management and SELinux relabeling.
</h6>

> spec > template > spec > volumes > [1] > iscsi = null

###### Volume's name.

 Must be a DNS_LABEL and unique within the pod. More info:
 http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > volumes > [1] > name = "backend-config"

###### Represents an NFS mount that lasts the lifetime of a pod.

 NFS volumes do not support
 ownership management or SELinux relabeling.

> spec > template > spec > volumes > [1] > nfs = null

###### PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace.

 This
 volume finds the bound PV and mounts that volume for the pod. A
 PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of
 volume that is owned by someone else (the system).

> spec > template > spec > volumes > [1] > persistentVolumeClaim = null

###### Represents a Photon Controller persistent disk resource.

> spec > template > spec > volumes > [1] > photonPersistentDisk = null

###### PortworxVolumeSource represents a Portworx volume resource.

> spec > template > spec > volumes > [1] > portworxVolume = null

###### Represents a projected volume source

> spec > template > spec > volumes > [1] > projected = null

###### Represents a Quobyte mount that lasts the lifetime of a pod.

 Quobyte volumes do not
 support ownership management or SELinux relabeling.

> spec > template > spec > volumes > [1] > quobyte = null

###### Represents a Rados Block Device mount that lasts the lifetime of a pod.

 RBD volumes
 support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [1] > rbd = null

###### ScaleIOVolumeSource represents a persistent ScaleIO volume

> spec > template > spec > volumes > [1] > scaleIO = null

###### Adapts a Secret into a volume.

 The contents of the target Secret's Data field will be presented in a volume as files
 using the keys in the Data field as the file names. Secret volumes support ownership
 management and SELinux relabeling.

> spec > template > spec > volumes > [1] > secret = null

###### Represents a vSphere volume resource.

> spec > template > spec > volumes > [1] > vsphereVolume = null

##### • `2`

###### Represents a Persistent Disk resource in AWS.

 An AWS EBS disk must exist before mounting to a container.

 The disk must also be in the
 same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS
 EBS volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [2] > awsElasticBlockStore = null

###### AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.

> spec > template > spec > volumes > [2] > azureDisk = null

###### AzureFile represents an Azure File Service mount on the host and bind mount to the pod.

> spec > template > spec > volumes > [2] > azureFile = null

<h6>
Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do notsupport ownership management or SELinux relabeling.
</h6>

> spec > template > spec > volumes > [2] > cephfs = null

###### Represents a cinder volume resource in Openstack.

 A Cinder volume must exist before
 mounting to a container. The volume must also be in the same region as the kubelet.
 Cinder volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [2] > cinder = null

###### Adapts a ConfigMap into a volume.

 The contents of the target ConfigMap's Data field will be presented in a volume as files
 using the keys in the Data field as the file names, unless the items element is populated
 with specific mappings of keys to paths.

 ConfigMap volumes support ownership management
 and SELinux relabeling.

###### Optional: mode bits to use on created files by default.

 Must be a value between 0 and
 0777. Defaults to 0644. Directories within the path are not affected by this setting.
 This might be in conflict with other options that affect the file mode, like fsGroup, and
 the result can be other mode bits set.

> spec > template > spec > volumes > [2] > configMap > defaultMode = null

<h6>
If unspecified, each key-value pair in the Data field of the referenced ConfigMap will beprojected into the volume as a file whose name is the key and content is the value.
</h6>

 If
 specified, the listed keys will be projected into the specified paths, and unlisted keys
 will not be present. If a key is specified which is not present in the ConfigMap, the
 volume setup will error unless it is marked optional. Paths must be relative and may not
 contain the '..' path or start with '..'.

> spec > template > spec > volumes > [2] > configMap > items = null

###### Name of the referent.

 More info: http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > volumes > [2] > configMap > name = "job-worker-config"

###### Specify whether the ConfigMap or it's keys must be defined

> spec > template > spec > volumes > [2] > configMap > optional = null

###### DownwardAPIVolumeSource represents a volume containing downward API info.

 Downward API
 volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [2] > downwardAPI = null

###### Represents an empty directory for a pod.

 Empty directory volumes support ownership
 management and SELinux relabeling.

> spec > template > spec > volumes > [2] > emptyDir = null

###### Represents a Fibre Channel volume.

 Fibre Channel volumes can only be mounted as
 read/write once. Fibre Channel volumes support ownership management and SELinux
 relabeling.

> spec > template > spec > volumes > [2] > fc = null

<h6>
FlexVolume represents a generic volume resource that is provisioned/attached using anexec based plugin.
</h6>

 This is an alpha feature and may change in future.

> spec > template > spec > volumes > [2] > flexVolume = null

###### Represents a Flocker volume mounted by the Flocker agent.

 One and only one of datasetName
 and datasetUUID should be set. Flocker volumes do not support ownership management or
 SELinux relabeling.

> spec > template > spec > volumes > [2] > flocker = null

###### Represents a Persistent Disk resource in Google Compute Engine.

 A GCE PD must exist before mounting to a container. The disk must also be in the same GCE
 project and zone as the kubelet. A GCE PD can only be mounted as read/write once or
 read-only many times. GCE PDs support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [2] > gcePersistentDisk = null

<h6>
Represents a volume that is populated with the contents of a git repository. Git repovolumes do not support ownership management.
</h6>

 Git repo volumes support SELinux relabeling.

> spec > template > spec > volumes > [2] > gitRepo = null

###### Represents a Glusterfs mount that lasts the lifetime of a pod.

 Glusterfs volumes do not
 support ownership management or SELinux relabeling.

> spec > template > spec > volumes > [2] > glusterfs = null

###### Represents a host path mapped into a pod.

 Host path volumes do not support ownership
 management or SELinux relabeling.

> spec > template > spec > volumes > [2] > hostPath = null

<h6>
Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSIvolumes support ownership management and SELinux relabeling.
</h6>

> spec > template > spec > volumes > [2] > iscsi = null

###### Volume's name.

 Must be a DNS_LABEL and unique within the pod. More info:
 http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > volumes > [2] > name = "job-worker-config"

###### Represents an NFS mount that lasts the lifetime of a pod.

 NFS volumes do not support
 ownership management or SELinux relabeling.

> spec > template > spec > volumes > [2] > nfs = null

###### PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace.

 This
 volume finds the bound PV and mounts that volume for the pod. A
 PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of
 volume that is owned by someone else (the system).

> spec > template > spec > volumes > [2] > persistentVolumeClaim = null

###### Represents a Photon Controller persistent disk resource.

> spec > template > spec > volumes > [2] > photonPersistentDisk = null

###### PortworxVolumeSource represents a Portworx volume resource.

> spec > template > spec > volumes > [2] > portworxVolume = null

###### Represents a projected volume source

> spec > template > spec > volumes > [2] > projected = null

###### Represents a Quobyte mount that lasts the lifetime of a pod.

 Quobyte volumes do not
 support ownership management or SELinux relabeling.

> spec > template > spec > volumes > [2] > quobyte = null

###### Represents a Rados Block Device mount that lasts the lifetime of a pod.

 RBD volumes
 support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [2] > rbd = null

###### ScaleIOVolumeSource represents a persistent ScaleIO volume

> spec > template > spec > volumes > [2] > scaleIO = null

###### Adapts a Secret into a volume.

 The contents of the target Secret's Data field will be presented in a volume as files
 using the keys in the Data field as the file names. Secret volumes support ownership
 management and SELinux relabeling.

> spec > template > spec > volumes > [2] > secret = null

###### Represents a vSphere volume resource.

> spec > template > spec > volumes > [2] > vsphereVolume = null

##### • `3`

###### Represents a Persistent Disk resource in AWS.

 An AWS EBS disk must exist before mounting to a container.

 The disk must also be in the
 same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS
 EBS volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [3] > awsElasticBlockStore = null

###### AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.

> spec > template > spec > volumes > [3] > azureDisk = null

###### AzureFile represents an Azure File Service mount on the host and bind mount to the pod.

> spec > template > spec > volumes > [3] > azureFile = null

<h6>
Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do notsupport ownership management or SELinux relabeling.
</h6>

> spec > template > spec > volumes > [3] > cephfs = null

###### Represents a cinder volume resource in Openstack.

 A Cinder volume must exist before
 mounting to a container. The volume must also be in the same region as the kubelet.
 Cinder volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [3] > cinder = null

###### Adapts a ConfigMap into a volume.

 The contents of the target ConfigMap's Data field will be presented in a volume as files
 using the keys in the Data field as the file names, unless the items element is populated
 with specific mappings of keys to paths.

 ConfigMap volumes support ownership management
 and SELinux relabeling.

> spec > template > spec > volumes > [3] > configMap = null

###### DownwardAPIVolumeSource represents a volume containing downward API info.

 Downward API
 volumes support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [3] > downwardAPI = null

###### Represents an empty directory for a pod.

 Empty directory volumes support ownership
 management and SELinux relabeling.

> spec > template > spec > volumes > [3] > emptyDir = null

###### Represents a Fibre Channel volume.

 Fibre Channel volumes can only be mounted as
 read/write once. Fibre Channel volumes support ownership management and SELinux
 relabeling.

> spec > template > spec > volumes > [3] > fc = null

<h6>
FlexVolume represents a generic volume resource that is provisioned/attached using anexec based plugin.
</h6>

 This is an alpha feature and may change in future.

> spec > template > spec > volumes > [3] > flexVolume = null

###### Represents a Flocker volume mounted by the Flocker agent.

 One and only one of datasetName
 and datasetUUID should be set. Flocker volumes do not support ownership management or
 SELinux relabeling.

> spec > template > spec > volumes > [3] > flocker = null

###### Represents a Persistent Disk resource in Google Compute Engine.

 A GCE PD must exist before mounting to a container. The disk must also be in the same GCE
 project and zone as the kubelet. A GCE PD can only be mounted as read/write once or
 read-only many times. GCE PDs support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [3] > gcePersistentDisk = null

<h6>
Represents a volume that is populated with the contents of a git repository. Git repovolumes do not support ownership management.
</h6>

 Git repo volumes support SELinux relabeling.

> spec > template > spec > volumes > [3] > gitRepo = null

###### Represents a Glusterfs mount that lasts the lifetime of a pod.

 Glusterfs volumes do not
 support ownership management or SELinux relabeling.

> spec > template > spec > volumes > [3] > glusterfs = null

###### Represents a host path mapped into a pod.

 Host path volumes do not support ownership
 management or SELinux relabeling.

> spec > template > spec > volumes > [3] > hostPath = null

<h6>
Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSIvolumes support ownership management and SELinux relabeling.
</h6>

> spec > template > spec > volumes > [3] > iscsi = null

###### Volume's name.

 Must be a DNS_LABEL and unique within the pod. More info:
 http://kubernetes.io/docs/user-guide/identifiers#names

> spec > template > spec > volumes > [3] > name = "shared-data"

###### Represents an NFS mount that lasts the lifetime of a pod.

 NFS volumes do not support
 ownership management or SELinux relabeling.

> spec > template > spec > volumes > [3] > nfs = null

###### PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace.

 This
 volume finds the bound PV and mounts that volume for the pod. A
 PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of
 volume that is owned by someone else (the system).

<h6>
ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod usingthis volume.
</h6>

 More info:
 http://kubernetes.io/docs/user-guide/persistent-volumes#persistentvolumeclaims

> spec > template > spec > volumes > [3] > persistentVolumeClaim > claimName = "shared-data-pvc"

###### Will force the ReadOnly setting in VolumeMounts.

 Default false.

> spec > template > spec > volumes > [3] > persistentVolumeClaim > readOnly = null

###### Represents a Photon Controller persistent disk resource.

> spec > template > spec > volumes > [3] > photonPersistentDisk = null

###### PortworxVolumeSource represents a Portworx volume resource.

> spec > template > spec > volumes > [3] > portworxVolume = null

###### Represents a projected volume source

> spec > template > spec > volumes > [3] > projected = null

###### Represents a Quobyte mount that lasts the lifetime of a pod.

 Quobyte volumes do not
 support ownership management or SELinux relabeling.

> spec > template > spec > volumes > [3] > quobyte = null

###### Represents a Rados Block Device mount that lasts the lifetime of a pod.

 RBD volumes
 support ownership management and SELinux relabeling.

> spec > template > spec > volumes > [3] > rbd = null

###### ScaleIOVolumeSource represents a persistent ScaleIO volume

> spec > template > spec > volumes > [3] > scaleIO = null

###### Adapts a Secret into a volume.

 The contents of the target Secret's Data field will be presented in a volume as files
 using the keys in the Data field as the file names. Secret volumes support ownership
 management and SELinux relabeling.

> spec > template > spec > volumes > [3] > secret = null

###### Represents a vSphere volume resource.

> spec > template > spec > volumes > [3] > vsphereVolume = null

# DeploymentStatus is the most recently observed status of the Deployment.

> status = null