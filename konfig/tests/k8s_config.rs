// NOTE: generated from https://raw.githubusercontent.com/garethr/kubernetes-json-schema/master/v1.6.1-standalone/deployment.json
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Deployment enables declarative updates for Pods and ReplicaSets.
#[derive(Serialize, Deserialize)]
pub struct Deployment {
    /// APIVersion defines the versioned schema of this representation of an object. Servers
    /// should convert recognized schemas to the latest internal value, and may reject
    /// unrecognized values. More info:
    /// http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    api_version: Option<String>,
    /// Kind is a string value representing the REST resource this object represents. Servers may
    /// infer this from the endpoint the client submits requests to. Cannot be updated. In
    /// CamelCase. More info:
    /// http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#types-kinds
    kind: Option<String>,
    /// ObjectMeta is metadata that all persisted resources must have, which includes all objects
    /// users must create.
    metadata: Option<DeploymentMetadata>,
    /// DeploymentSpec is the specification of the desired behavior of the Deployment.
    spec: Option<DeploymentSpec>,
    /// DeploymentStatus is the most recently observed status of the Deployment.
    status: Option<StatusUnion>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be set by
    /// external tools to store and retrieve arbitrary metadata. They are not queryable and
    /// should be preserved when modifying objects. More info:
    /// http://kubernetes.io/docs/user-guide/annotations
    annotations: Option<HashMap<String, Option<String>>>,
    /// The name of the cluster which the object belongs to. This is used to distinguish
    /// resources with same name and namespace in different clusters. This field is not set
    /// anywhere right now and apiserver is going to ignore it if set in create or update request.
    #[serde(rename = "clusterName")]
    cluster_name: Option<String>,
    #[serde(rename = "creationTimestamp")]
    creation_timestamp: Option<String>,
    /// Number of seconds allowed for this object to gracefully terminate before it will be
    /// removed from the system. Only set when deletionTimestamp is also set. May only be
    /// shortened. Read-only.
    #[serde(rename = "deletionGracePeriodSeconds")]
    deletion_grace_period_seconds: Option<i64>,
    #[serde(rename = "deletionTimestamp")]
    deletion_timestamp: Option<String>,
    /// Must be empty before the object is deleted from the registry. Each entry is an identifier
    /// for the responsible component that will remove the entry from the list. If the
    /// deletionTimestamp of the object is non-nil, entries in this list can only be removed.
    finalizers: Option<Vec<Option<String>>>,
    /// GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF
    /// the Name field has not been provided. If this field is used, the name returned to the
    /// client will be different than the name passed. This value will also be combined with a
    /// unique suffix. The provided value has the same validation rules as the Name field, and
    /// may be truncated by the length of the suffix required to make the value unique on the
    /// server.
    ///
    /// If this field is specified and the generated name exists, the server will NOT return a
    /// 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout
    /// indicating a unique name could not be found in the time allotted, and the client should
    /// retry (optionally after the time indicated in the Retry-After header).
    ///
    /// Applied only if Name is not specified. More info:
    /// http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#idempotency
    #[serde(rename = "generateName")]
    generate_name: Option<String>,
    /// A sequence number representing a specific generation of the desired state. Populated by
    /// the system. Read-only.
    generation: Option<i64>,
    /// Map of string keys and values that can be used to organize and categorize (scope and
    /// select) objects. May match selectors of replication controllers and services. More info:
    /// http://kubernetes.io/docs/user-guide/labels
    labels: Option<HashMap<String, Option<String>>>,
    /// Name must be unique within a namespace. Is required when creating resources, although
    /// some resources may allow a client to request the generation of an appropriate name
    /// automatically. Name is primarily intended for creation idempotence and configuration
    /// definition. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Namespace defines the space within each name must be unique. An empty namespace is
    /// equivalent to the "default" namespace, but "default" is the canonical representation. Not
    /// all objects are required to be scoped to a namespace - the value of this field for those
    /// objects will be empty.
    ///
    /// Must be a DNS_LABEL. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/namespaces
    namespace: Option<String>,
    /// List of objects depended by this object. If ALL objects in the list have been deleted,
    /// this object will be garbage collected. If this object is managed by a controller, then an
    /// entry in this list will point to this controller, with the controller field set to true.
    /// There cannot be more than one managing controller.
    #[serde(rename = "ownerReferences")]
    owner_references: Option<Vec<Option<TentacledOwnerReference>>>,
    /// An opaque value that represents the internal version of this object that can be used by
    /// clients to determine when objects have changed. May be used for optimistic concurrency,
    /// change detection, and the watch operation on a resource or set of resources. Clients must
    /// treat these values as opaque and passed unmodified back to the server. They may only be
    /// valid for a particular resource or set of resources.
    ///
    /// Populated by the system. Read-only. Value must be treated as opaque by clients and . More
    /// info:
    /// http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#concurrency-control-and-consistency
    #[serde(rename = "resourceVersion")]
    resource_version: Option<String>,
    /// SelfLink is a URL representing this object. Populated by the system. Read-only.
    #[serde(rename = "selfLink")]
    self_link: Option<String>,
    /// UID is the unique in time and space value for this object. It is typically generated by
    /// the server on successful creation of a resource and is not allowed to change on PUT
    /// operations.
    ///
    /// Populated by the system. Read-only. More info:
    /// http://kubernetes.io/docs/user-guide/identifiers#uids
    uid: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleOwnerReference {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    api_version: String,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot
    /// be deleted from the key-value store until this reference is removed. Defaults to false.
    /// To set this field, a user needs "delete" permission of the owner, otherwise 422
    /// (Unprocessable Entity) will be returned.
    #[serde(rename = "blockOwnerDeletion")]
    block_owner_deletion: Option<bool>,
    /// If true, this reference points to the managing controller.
    controller: Option<bool>,
    /// Kind of the referent. More info:
    /// http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#types-kinds
    kind: String,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: String,
    /// UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    uid: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSpec {
    /// Minimum number of seconds for which a newly created pod should be ready without any of
    /// its container crashing, for it to be considered available. Defaults to 0 (pod will be
    /// considered available as soon as it is ready)
    #[serde(rename = "minReadySeconds")]
    min_ready_seconds: Option<i64>,
    /// Indicates that the deployment is paused.
    paused: Option<bool>,
    /// The maximum time in seconds for a deployment to make progress before it is considered to
    /// be failed. The deployment controller will continue to process failed deployments and a
    /// condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment
    /// status. Once autoRollback is implemented, the deployment controller will automatically
    /// rollback failed deployments. Note that progress will not be estimated during the time a
    /// deployment is paused. Defaults to 600s.
    #[serde(rename = "progressDeadlineSeconds")]
    progress_deadline_seconds: Option<i64>,
    /// Number of desired pods. This is a pointer to distinguish between explicit zero and not
    /// specified. Defaults to 1.
    replicas: Option<i64>,
    /// The number of old ReplicaSets to retain to allow rollback. This is a pointer to
    /// distinguish between explicit zero and not specified. Defaults to 2.
    #[serde(rename = "revisionHistoryLimit")]
    revision_history_limit: Option<i64>,
    #[serde(rename = "rollbackTo")]
    rollback_to: Option<RollbackToUnion>,
    /// A label selector is a label query over a set of resources. The result of matchLabels and
    /// matchExpressions are ANDed. An empty label selector matches all objects. A null label
    /// selector matches no objects.
    selector: Option<SelectorUnion>,
    /// DeploymentStrategy describes how to replace existing pods with new ones.
    strategy: Option<StrategyUnion>,
    /// PodTemplateSpec describes the data a pod should have when created from a template
    template: Option<TemplateUnion>,
}

#[derive(Serialize, Deserialize)]
pub struct RollbackToClass {
    /// The revision to rollback to. If set to 0, rollbck to the last revision.
    revision: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorClass {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    match_expressions: Option<Vec<Option<SelectorMatchExpression>>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is
    /// equivalent to an element of matchExpressions, whose key field is "key", the operator is
    /// "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(rename = "matchLabels")]
    match_labels: Option<HashMap<String, Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleMatchExpression {
    /// key is the label key that the selector applies to.
    key: String,
    /// operator represents a key's relationship to a set of values. Valid operators ard In,
    /// NotIn, Exists and DoesNotExist.
    operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array
    /// must be non-empty. If the operator is Exists or DoesNotExist, the values array must be
    /// empty. This array is replaced during a strategic merge patch.
    values: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct StrategyClass {
    /// Spec to control the desired behavior of rolling update.
    #[serde(rename = "rollingUpdate")]
    rolling_update: Option<RollingUpdateUnion>,
    /// Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
    #[serde(rename = "type")]
    strategy_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RollingUpdateClass {
    #[serde(rename = "maxSurge")]
    max_surge: Option<MaxSurge>,
    #[serde(rename = "maxUnavailable")]
    max_unavailable: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct TemplateClass {
    /// ObjectMeta is metadata that all persisted resources must have, which includes all objects
    /// users must create.
    metadata: Option<TemplateMetadata>,
    /// PodSpec is a description of a pod.
    spec: Option<TemplateSpec>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyMetadata {
    /// Annotations is an unstructured key value map stored with a resource that may be set by
    /// external tools to store and retrieve arbitrary metadata. They are not queryable and
    /// should be preserved when modifying objects. More info:
    /// http://kubernetes.io/docs/user-guide/annotations
    annotations: Option<HashMap<String, Option<String>>>,
    /// The name of the cluster which the object belongs to. This is used to distinguish
    /// resources with same name and namespace in different clusters. This field is not set
    /// anywhere right now and apiserver is going to ignore it if set in create or update request.
    #[serde(rename = "clusterName")]
    cluster_name: Option<String>,
    #[serde(rename = "creationTimestamp")]
    creation_timestamp: Option<String>,
    /// Number of seconds allowed for this object to gracefully terminate before it will be
    /// removed from the system. Only set when deletionTimestamp is also set. May only be
    /// shortened. Read-only.
    #[serde(rename = "deletionGracePeriodSeconds")]
    deletion_grace_period_seconds: Option<i64>,
    #[serde(rename = "deletionTimestamp")]
    deletion_timestamp: Option<String>,
    /// Must be empty before the object is deleted from the registry. Each entry is an identifier
    /// for the responsible component that will remove the entry from the list. If the
    /// deletionTimestamp of the object is non-nil, entries in this list can only be removed.
    finalizers: Option<Vec<Option<String>>>,
    /// GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF
    /// the Name field has not been provided. If this field is used, the name returned to the
    /// client will be different than the name passed. This value will also be combined with a
    /// unique suffix. The provided value has the same validation rules as the Name field, and
    /// may be truncated by the length of the suffix required to make the value unique on the
    /// server.
    ///
    /// If this field is specified and the generated name exists, the server will NOT return a
    /// 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout
    /// indicating a unique name could not be found in the time allotted, and the client should
    /// retry (optionally after the time indicated in the Retry-After header).
    ///
    /// Applied only if Name is not specified. More info:
    /// http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#idempotency
    #[serde(rename = "generateName")]
    generate_name: Option<String>,
    /// A sequence number representing a specific generation of the desired state. Populated by
    /// the system. Read-only.
    generation: Option<i64>,
    /// Map of string keys and values that can be used to organize and categorize (scope and
    /// select) objects. May match selectors of replication controllers and services. More info:
    /// http://kubernetes.io/docs/user-guide/labels
    labels: Option<HashMap<String, Option<String>>>,
    /// Name must be unique within a namespace. Is required when creating resources, although
    /// some resources may allow a client to request the generation of an appropriate name
    /// automatically. Name is primarily intended for creation idempotence and configuration
    /// definition. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Namespace defines the space within each name must be unique. An empty namespace is
    /// equivalent to the "default" namespace, but "default" is the canonical representation. Not
    /// all objects are required to be scoped to a namespace - the value of this field for those
    /// objects will be empty.
    ///
    /// Must be a DNS_LABEL. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/namespaces
    namespace: Option<String>,
    /// List of objects depended by this object. If ALL objects in the list have been deleted,
    /// this object will be garbage collected. If this object is managed by a controller, then an
    /// entry in this list will point to this controller, with the controller field set to true.
    /// There cannot be more than one managing controller.
    #[serde(rename = "ownerReferences")]
    owner_references: Option<Vec<Option<StickyOwnerReference>>>,
    /// An opaque value that represents the internal version of this object that can be used by
    /// clients to determine when objects have changed. May be used for optimistic concurrency,
    /// change detection, and the watch operation on a resource or set of resources. Clients must
    /// treat these values as opaque and passed unmodified back to the server. They may only be
    /// valid for a particular resource or set of resources.
    ///
    /// Populated by the system. Read-only. Value must be treated as opaque by clients and . More
    /// info:
    /// http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#concurrency-control-and-consistency
    #[serde(rename = "resourceVersion")]
    resource_version: Option<String>,
    /// SelfLink is a URL representing this object. Populated by the system. Read-only.
    #[serde(rename = "selfLink")]
    self_link: Option<String>,
    /// UID is the unique in time and space value for this object. It is typically generated by
    /// the server on successful creation of a resource and is not allowed to change on PUT
    /// operations.
    ///
    /// Populated by the system. Read-only. More info:
    /// http://kubernetes.io/docs/user-guide/identifiers#uids
    uid: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyOwnerReference {
    /// API version of the referent.
    #[serde(rename = "apiVersion")]
    api_version: String,
    /// If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot
    /// be deleted from the key-value store until this reference is removed. Defaults to false.
    /// To set this field, a user needs "delete" permission of the owner, otherwise 422
    /// (Unprocessable Entity) will be returned.
    #[serde(rename = "blockOwnerDeletion")]
    block_owner_deletion: Option<bool>,
    /// If true, this reference points to the managing controller.
    controller: Option<bool>,
    /// Kind of the referent. More info:
    /// http://releases.k8s.io/HEAD/docs/devel/api-conventions.md#types-kinds
    kind: String,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: String,
    /// UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
    uid: String,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySpec {
    /// Optional duration in seconds the pod may be active on the node relative to StartTime
    /// before the system will actively try to mark it failed and kill associated containers.
    /// Value must be a positive integer.
    #[serde(rename = "activeDeadlineSeconds")]
    active_deadline_seconds: Option<i64>,
    /// Affinity is a group of affinity scheduling rules.
    affinity: Option<AffinityUnion>,
    /// AutomountServiceAccountToken indicates whether a service account token should be
    /// automatically mounted.
    #[serde(rename = "automountServiceAccountToken")]
    automount_service_account_token: Option<bool>,
    /// List of containers belonging to the pod. Containers cannot currently be added or removed.
    /// There must be at least one container in a Pod. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/containers
    containers: Vec<Option<ContainerElement>>,
    /// Set DNS policy for containers within the pod. One of 'ClusterFirstWithHostNet',
    /// 'ClusterFirst' or 'Default'. Defaults to "ClusterFirst". To have DNS options set along
    /// with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
    #[serde(rename = "dnsPolicy")]
    dns_policy: Option<String>,
    /// Use the host's ipc namespace. Optional: Default to false.
    #[serde(rename = "hostIPC")]
    host_ipc: Option<bool>,
    /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a
    /// system-defined value.
    hostname: Option<String>,
    /// Host networking requested for this pod. Use the host's network namespace. If this option
    /// is set, the ports that will be used must be specified. Default to false.
    #[serde(rename = "hostNetwork")]
    host_network: Option<bool>,
    /// Use the host's pid namespace. Optional: Default to false.
    #[serde(rename = "hostPID")]
    host_pid: Option<bool>,
    /// ImagePullSecrets is an optional list of references to secrets in the same namespace to
    /// use for pulling any of the images used by this PodSpec. If specified, these secrets will
    /// be passed to individual puller implementations for them to use. For example, in the case
    /// of docker, only DockerConfig type secrets are honored. More info:
    /// http://kubernetes.io/docs/user-guide/images#specifying-imagepullsecrets-on-a-pod
    #[serde(rename = "imagePullSecrets")]
    image_pull_secrets: Option<Vec<Option<ImagePullSecretElement>>>,
    /// List of initialization containers belonging to the pod. Init containers are executed in
    /// order prior to containers being started. If any init container fails, the pod is
    /// considered to have failed and is handled according to its restartPolicy. The name for an
    /// init container or normal container must be unique among all containers. Init containers
    /// may not have Lifecycle actions, Readiness probes, or Liveness probes. The
    /// resourceRequirements of an init container are taken into account during scheduling by
    /// finding the highest request/limit for each resource type, and then using the max of of
    /// that value or the sum of the normal containers. Limits are applied to init containers in
    /// a similar fashion. Init containers cannot currently be added or removed. Cannot be
    /// updated. More info: http://kubernetes.io/docs/user-guide/containers
    #[serde(rename = "initContainers")]
    init_containers: Option<Vec<Option<InitContainerElement>>>,
    /// NodeName is a request to schedule this pod onto a specific node. If it is non-empty, the
    /// scheduler simply schedules this pod onto that node, assuming that it fits resource
    /// requirements.
    #[serde(rename = "nodeName")]
    node_name: Option<String>,
    /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector
    /// which must match a node's labels for the pod to be scheduled on that node. More info:
    /// http://kubernetes.io/docs/user-guide/node-selection/README
    #[serde(rename = "nodeSelector")]
    node_selector: Option<HashMap<String, Option<String>>>,
    /// Restart policy for all containers within the pod. One of Always, OnFailure, Never.
    /// Default to Always. More info:
    /// http://kubernetes.io/docs/user-guide/pod-states#restartpolicy
    #[serde(rename = "restartPolicy")]
    restart_policy: Option<String>,
    /// If specified, the pod will be dispatched by specified scheduler. If not specified, the
    /// pod will be dispatched by default scheduler.
    #[serde(rename = "schedulerName")]
    scheduler_name: Option<String>,
    /// PodSecurityContext holds pod-level security attributes and common container settings.
    /// Some fields are also present in container.securityContext.  Field values of
    /// container.securityContext take precedence over field values of PodSecurityContext.
    #[serde(rename = "securityContext")]
    security_context: Option<SpecSecurityContext>,
    /// DeprecatedServiceAccount is a depreciated alias for ServiceAccountName. Deprecated: Use
    /// serviceAccountName instead.
    #[serde(rename = "serviceAccount")]
    service_account: Option<String>,
    /// ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info:
    /// http://releases.k8s.io/HEAD/docs/design/service_accounts.md
    #[serde(rename = "serviceAccountName")]
    service_account_name: Option<String>,
    /// If specified, the fully qualified Pod hostname will be "<hostname>.<subdomain>.<pod
    /// namespace>.svc.<cluster domain>". If not specified, the pod will not have a domainname at
    /// all.
    subdomain: Option<String>,
    /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in
    /// delete request. Value must be non-negative integer. The value zero indicates delete
    /// immediately. If this value is nil, the default grace period will be used instead. The
    /// grace period is the duration in seconds after the processes running in the pod are sent a
    /// termination signal and the time when the processes are forcibly halted with a kill
    /// signal. Set this value longer than the expected cleanup time for your process. Defaults
    /// to 30 seconds.
    #[serde(rename = "terminationGracePeriodSeconds")]
    termination_grace_period_seconds: Option<i64>,
    /// If specified, the pod's tolerations.
    tolerations: Option<Vec<Option<TolerationElement>>>,
    /// List of volumes that can be mounted by containers belonging to the pod. More info:
    /// http://kubernetes.io/docs/user-guide/volumes
    volumes: Option<Vec<Option<VolumeElement>>>,
}

#[derive(Serialize, Deserialize)]
pub struct AffinityClass {
    /// Node affinity is a group of node affinity scheduling rules.
    #[serde(rename = "nodeAffinity")]
    node_affinity: Option<NodeAffinityUnion>,
    /// Pod affinity is a group of inter pod affinity scheduling rules.
    #[serde(rename = "podAffinity")]
    pod_affinity: Option<PodAffinityUnion>,
    /// Pod anti affinity is a group of inter pod anti affinity scheduling rules.
    #[serde(rename = "podAntiAffinity")]
    pod_anti_affinity: Option<PodAntiAffinityUnion>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeAffinityClass {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions
    /// specified by this field, but it may choose a node that violates one or more of the
    /// expressions. The node that is most preferred is the one with the greatest sum of weights,
    /// i.e. for each node that meets all of the scheduling requirements (resource request,
    /// requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through
    /// the elements of this field and adding "weight" to the sum if the node matches the
    /// corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
    #[serde(rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    preferred_during_scheduling_ignored_during_execution:
        Option<Vec<Option<NodeAffinityPreferredDuringSchedulingIgnoredDuringExecution>>>,
    /// A node selector represents the union of the results of one or more label queries over a
    /// set of nodes; that is, it represents the OR of the selectors represented by the node
    /// selector terms.
    #[serde(rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    required_during_scheduling_ignored_during_execution:
        Option<NodeAffinityRequiredDuringSchedulingIgnoredDuringExecution>,
}

#[derive(Serialize, Deserialize)]
pub struct PurplePreferredDuringSchedulingIgnoredDuringExecution {
    /// A null or empty node selector term matches no objects.
    preference: Option<PreferenceUnion>,
    /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
    weight: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PreferenceClass {
    /// Required. A list of node selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    match_expressions: Vec<Option<PreferenceMatchExpression>>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyMatchExpression {
    /// The label key that the selector applies to.
    key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn,
    /// Exists, DoesNotExist. Gt, and Lt.
    operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be
    /// non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If
    /// the operator is Gt or Lt, the values array must have a single element, which will be
    /// interpreted as an integer. This array is replaced during a strategic merge patch.
    values: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleRequiredDuringSchedulingIgnoredDuringExecution {
    /// Required. A list of node selector terms. The terms are ORed.
    #[serde(rename = "nodeSelectorTerms")]
    node_selector_terms: Vec<Option<NodeSelectorTermElement>>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeSelectorTermClass {
    /// Required. A list of node selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    match_expressions: Vec<Option<NodeSelectorTermMatchExpression>>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledMatchExpression {
    /// The label key that the selector applies to.
    key: String,
    /// Represents a key's relationship to a set of values. Valid operators are In, NotIn,
    /// Exists, DoesNotExist. Gt, and Lt.
    operator: String,
    /// An array of string values. If the operator is In or NotIn, the values array must be
    /// non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If
    /// the operator is Gt or Lt, the values array must have a single element, which will be
    /// interpreted as an integer. This array is replaced during a strategic merge patch.
    values: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PodAffinityClass {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions
    /// specified by this field, but it may choose a node that violates one or more of the
    /// expressions. The node that is most preferred is the one with the greatest sum of weights,
    /// i.e. for each node that meets all of the scheduling requirements (resource request,
    /// requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through
    /// the elements of this field and adding "weight" to the sum if the node has pods which
    /// matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most
    /// preferred.
    #[serde(rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    preferred_during_scheduling_ignored_during_execution:
        Option<Vec<Option<PodAffinityPreferredDuringSchedulingIgnoredDuringExecution>>>,
    /// NOT YET IMPLEMENTED. TODO: Uncomment field once it is implemented. If the affinity
    /// requirements specified by this field are not met at scheduling time, the pod will not be
    /// scheduled onto the node. If the affinity requirements specified by this field cease to be
    /// met at some point during pod execution (e.g. due to a pod label update), the system will
    /// try to eventually evict the pod from its node. When there are multiple elements, the
    /// lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must
    /// be satisfied. RequiredDuringSchedulingRequiredDuringExecution []PodAffinityTerm
    /// `json:"requiredDuringSchedulingRequiredDuringExecution,omitempty"` If the affinity
    /// requirements specified by this field are not met at scheduling time, the pod will not be
    /// scheduled onto the node. If the affinity requirements specified by this field cease to be
    /// met at some point during pod execution (e.g. due to a pod label update), the system may
    /// or may not try to eventually evict the pod from its node. When there are multiple
    /// elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e.
    /// all terms must be satisfied.
    #[serde(rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    required_during_scheduling_ignored_during_execution:
        Option<Vec<Option<PodAffinityRequiredDuringSchedulingIgnoredDuringExecution>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyPreferredDuringSchedulingIgnoredDuringExecution {
    /// Defines a set of pods (namely those matching the labelSelector relative to the given
    /// namespace(s)) that this pod should be co-located (affinity) or not co-located
    /// (anti-affinity) with, where co-located is defined as running on a node whose value of the
    /// label with key <topologyKey> tches that of any node on which a pod of the set of pods is
    /// running
    #[serde(rename = "podAffinityTerm")]
    pod_affinity_term: Option<TentacledPodAffinityTerm>,
    /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
    weight: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PurplePodAffinityTerm {
    /// A label selector is a label query over a set of resources. The result of matchLabels and
    /// matchExpressions are ANDed. An empty label selector matches all objects. A null label
    /// selector matches no objects.
    #[serde(rename = "labelSelector")]
    label_selector: Option<IndigoLabelSelector>,
    /// namespaces specifies which namespaces the labelSelector applies to (matches against);
    /// null or empty list means "this pod's namespace"
    namespaces: Option<Vec<Option<String>>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods
    /// matching the labelSelector in the specified namespaces, where co-located is defined as
    /// running on a node whose value of the label with key topologyKey matches that of any node
    /// on which any of the selected pods is running. For PreferredDuringScheduling pod
    /// anti-affinity, empty topologyKey is interpreted as "all topologies" ("all topologies"
    /// here means all the topologyKeys indicated by scheduler command-line argument
    /// --failure-domains); for affinity and for RequiredDuringScheduling pod anti-affinity,
    /// empty topologyKey is not allowed.
    #[serde(rename = "topologyKey")]
    topology_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    match_expressions: Option<Vec<Option<AmbitiousMatchExpression>>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is
    /// equivalent to an element of matchExpressions, whose key field is "key", the operator is
    /// "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(rename = "matchLabels")]
    match_labels: Option<HashMap<String, Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyMatchExpression {
    /// key is the label key that the selector applies to.
    key: String,
    /// operator represents a key's relationship to a set of values. Valid operators ard In,
    /// NotIn, Exists and DoesNotExist.
    operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array
    /// must be non-empty. If the operator is Exists or DoesNotExist, the values array must be
    /// empty. This array is replaced during a strategic merge patch.
    values: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyRequiredDuringSchedulingIgnoredDuringExecution {
    /// A label selector is a label query over a set of resources. The result of matchLabels and
    /// matchExpressions are ANDed. An empty label selector matches all objects. A null label
    /// selector matches no objects.
    #[serde(rename = "labelSelector")]
    label_selector: Option<IndecentLabelSelector>,
    /// namespaces specifies which namespaces the labelSelector applies to (matches against);
    /// null or empty list means "this pod's namespace"
    namespaces: Option<Vec<Option<String>>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods
    /// matching the labelSelector in the specified namespaces, where co-located is defined as
    /// running on a node whose value of the label with key topologyKey matches that of any node
    /// on which any of the selected pods is running. For PreferredDuringScheduling pod
    /// anti-affinity, empty topologyKey is interpreted as "all topologies" ("all topologies"
    /// here means all the topologyKeys indicated by scheduler command-line argument
    /// --failure-domains); for affinity and for RequiredDuringScheduling pod anti-affinity,
    /// empty topologyKey is not allowed.
    #[serde(rename = "topologyKey")]
    topology_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    match_expressions: Option<Vec<Option<CunningMatchExpression>>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is
    /// equivalent to an element of matchExpressions, whose key field is "key", the operator is
    /// "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(rename = "matchLabels")]
    match_labels: Option<HashMap<String, Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoMatchExpression {
    /// key is the label key that the selector applies to.
    key: String,
    /// operator represents a key's relationship to a set of values. Valid operators ard In,
    /// NotIn, Exists and DoesNotExist.
    operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array
    /// must be non-empty. If the operator is Exists or DoesNotExist, the values array must be
    /// empty. This array is replaced during a strategic merge patch.
    values: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PodAntiAffinityClass {
    /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity
    /// expressions specified by this field, but it may choose a node that violates one or more
    /// of the expressions. The node that is most preferred is the one with the greatest sum of
    /// weights, i.e. for each node that meets all of the scheduling requirements (resource
    /// request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by
    /// iterating through the elements of this field and adding "weight" to the sum if the node
    /// has pods which matches the corresponding podAffinityTerm; the node(s) with the highest
    /// sum are the most preferred.
    #[serde(rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    preferred_during_scheduling_ignored_during_execution:
        Option<Vec<Option<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution>>>,
    /// NOT YET IMPLEMENTED. TODO: Uncomment field once it is implemented. If the anti-affinity
    /// requirements specified by this field are not met at scheduling time, the pod will not be
    /// scheduled onto the node. If the anti-affinity requirements specified by this field cease
    /// to be met at some point during pod execution (e.g. due to a pod label update), the system
    /// will try to eventually evict the pod from its node. When there are multiple elements, the
    /// lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must
    /// be satisfied. RequiredDuringSchedulingRequiredDuringExecution []PodAffinityTerm
    /// `json:"requiredDuringSchedulingRequiredDuringExecution,omitempty"` If the anti-affinity
    /// requirements specified by this field are not met at scheduling time, the pod will not be
    /// scheduled onto the node. If the anti-affinity requirements specified by this field cease
    /// to be met at some point during pod execution (e.g. due to a pod label update), the system
    /// may or may not try to eventually evict the pod from its node. When there are multiple
    /// elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e.
    /// all terms must be satisfied.
    #[serde(rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    required_during_scheduling_ignored_during_execution:
        Option<Vec<Option<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution>>>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledPreferredDuringSchedulingIgnoredDuringExecution {
    /// Defines a set of pods (namely those matching the labelSelector relative to the given
    /// namespace(s)) that this pod should be co-located (affinity) or not co-located
    /// (anti-affinity) with, where co-located is defined as running on a node whose value of the
    /// label with key <topologyKey> tches that of any node on which a pod of the set of pods is
    /// running
    #[serde(rename = "podAffinityTerm")]
    pod_affinity_term: Option<StickyPodAffinityTerm>,
    /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
    weight: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyPodAffinityTerm {
    /// A label selector is a label query over a set of resources. The result of matchLabels and
    /// matchExpressions are ANDed. An empty label selector matches all objects. A null label
    /// selector matches no objects.
    #[serde(rename = "labelSelector")]
    label_selector: Option<HilariousLabelSelector>,
    /// namespaces specifies which namespaces the labelSelector applies to (matches against);
    /// null or empty list means "this pod's namespace"
    namespaces: Option<Vec<Option<String>>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods
    /// matching the labelSelector in the specified namespaces, where co-located is defined as
    /// running on a node whose value of the label with key topologyKey matches that of any node
    /// on which any of the selected pods is running. For PreferredDuringScheduling pod
    /// anti-affinity, empty topologyKey is interpreted as "all topologies" ("all topologies"
    /// here means all the topologyKeys indicated by scheduler command-line argument
    /// --failure-domains); for affinity and for RequiredDuringScheduling pod anti-affinity,
    /// empty topologyKey is not allowed.
    #[serde(rename = "topologyKey")]
    topology_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    match_expressions: Option<Vec<Option<MagentaMatchExpression>>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is
    /// equivalent to an element of matchExpressions, whose key field is "key", the operator is
    /// "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(rename = "matchLabels")]
    match_labels: Option<HashMap<String, Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentMatchExpression {
    /// key is the label key that the selector applies to.
    key: String,
    /// operator represents a key's relationship to a set of values. Valid operators ard In,
    /// NotIn, Exists and DoesNotExist.
    operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array
    /// must be non-empty. If the operator is Exists or DoesNotExist, the values array must be
    /// empty. This array is replaced during a strategic merge patch.
    values: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledRequiredDuringSchedulingIgnoredDuringExecution {
    /// A label selector is a label query over a set of resources. The result of matchLabels and
    /// matchExpressions are ANDed. An empty label selector matches all objects. A null label
    /// selector matches no objects.
    #[serde(rename = "labelSelector")]
    label_selector: Option<AmbitiousLabelSelector>,
    /// namespaces specifies which namespaces the labelSelector applies to (matches against);
    /// null or empty list means "this pod's namespace"
    namespaces: Option<Vec<Option<String>>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods
    /// matching the labelSelector in the specified namespaces, where co-located is defined as
    /// running on a node whose value of the label with key topologyKey matches that of any node
    /// on which any of the selected pods is running. For PreferredDuringScheduling pod
    /// anti-affinity, empty topologyKey is interpreted as "all topologies" ("all topologies"
    /// here means all the topologyKeys indicated by scheduler command-line argument
    /// --failure-domains); for affinity and for RequiredDuringScheduling pod anti-affinity,
    /// empty topologyKey is not allowed.
    #[serde(rename = "topologyKey")]
    topology_key: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyLabelSelector {
    /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
    #[serde(rename = "matchExpressions")]
    match_expressions: Option<Vec<Option<FriskyMatchExpression>>>,
    /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is
    /// equivalent to an element of matchExpressions, whose key field is "key", the operator is
    /// "In", and the values array contains only "value". The requirements are ANDed.
    #[serde(rename = "matchLabels")]
    match_labels: Option<HashMap<String, Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousMatchExpression {
    /// key is the label key that the selector applies to.
    key: String,
    /// operator represents a key's relationship to a set of values. Valid operators ard In,
    /// NotIn, Exists and DoesNotExist.
    operator: String,
    /// values is an array of string values. If the operator is In or NotIn, the values array
    /// must be non-empty. If the operator is Exists or DoesNotExist, the values array must be
    /// empty. This array is replaced during a strategic merge patch.
    values: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct ContainerClass {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided.
    /// Variable references $(VAR_NAME) are expanded using the container's environment. If a
    /// variable cannot be resolved, the reference in the input string will be unchanged. The
    /// $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references
    /// will never be expanded, regardless of whether the variable exists or not. Cannot be
    /// updated. More info:
    /// http://kubernetes.io/docs/user-guide/containers#containers-and-commands
    args: Option<Vec<Option<String>>>,
    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if
    /// this is not provided. Variable references $(VAR_NAME) are expanded using the container's
    /// environment. If a variable cannot be resolved, the reference in the input string will be
    /// unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME).
    /// Escaped references will never be expanded, regardless of whether the variable exists or
    /// not. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/containers#containers-and-commands
    command: Option<Vec<Option<String>>>,
    /// List of environment variables to set in the container. Cannot be updated.
    env: Option<Vec<Option<ContainerEnv>>>,
    /// List of sources to populate environment variables in the container. The keys defined
    /// within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event
    /// when the container is starting. When a key exists in multiple sources, the value
    /// associated with the last source will take precedence. Values defined by an Env with a
    /// duplicate key will take precedence. Cannot be updated.
    #[serde(rename = "envFrom")]
    env_from: Option<Vec<Option<ContainerEnvFrom>>>,
    /// Docker image name. More info: http://kubernetes.io/docs/user-guide/images
    image: Option<String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag
    /// is specified, or IfNotPresent otherwise. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/images#updating-images
    #[serde(rename = "imagePullPolicy")]
    image_pull_policy: Option<String>,
    /// Lifecycle describes actions that the management system should take in response to
    /// container lifecycle events. For the PostStart and PreStop lifecycle handlers, management
    /// of the container blocks until the action is complete, unless the container process fails,
    /// in which case the handler is aborted.
    lifecycle: Option<ContainerLifecycle>,
    /// Probe describes a health check to be performed against a container to determine whether
    /// it is alive or ready to receive traffic.
    #[serde(rename = "livenessProbe")]
    liveness_probe: Option<ContainerLivenessProbe>,
    /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a
    /// unique name (DNS_LABEL). Cannot be updated.
    name: String,
    /// List of ports to expose from the container. Exposing a port here gives the system
    /// additional information about the network connections a container uses, but is primarily
    /// informational. Not specifying a port here DOES NOT prevent that port from being exposed.
    /// Any port which is listening on the default "0.0.0.0" address inside a container will be
    /// accessible from the network. Cannot be updated.
    ports: Option<Vec<Option<ContainerPort>>>,
    /// Probe describes a health check to be performed against a container to determine whether
    /// it is alive or ready to receive traffic.
    #[serde(rename = "readinessProbe")]
    readiness_probe: Option<ContainerReadinessProbe>,
    /// ResourceRequirements describes the compute resource requirements.
    resources: Option<ContainerResources>,
    /// SecurityContext holds security configuration that will be applied to a container. Some
    /// fields are present in both SecurityContext and PodSecurityContext.  When both are set,
    /// the values in SecurityContext take precedence.
    #[serde(rename = "securityContext")]
    security_context: Option<ContainerSecurityContext>,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If
    /// this is not set, reads from stdin in the container will always result in EOF. Default is
    /// false.
    stdin: Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by
    /// a single attach. When stdin is true the stdin stream will remain open across multiple
    /// attach sessions. If stdinOnce is set to true, stdin is opened on container start, is
    /// empty until the first client attaches to stdin, and then remains open and accepts data
    /// until the client disconnects, at which time stdin is closed and remains closed until the
    /// container is restarted. If this flag is false, a container processes that reads from
    /// stdin will never receive an EOF. Default is false
    #[serde(rename = "stdinOnce")]
    stdin_once: Option<bool>,
    /// Optional: Path at which the file to which the container's termination message will be
    /// written is mounted into the container's filesystem. Message written is intended to be
    /// brief final status, such as an assertion failure message. Will be truncated by the node
    /// if greater than 4096 bytes. The total message length across all containers will be
    /// limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    #[serde(rename = "terminationMessagePath")]
    termination_message_path: Option<String>,
    /// Indicate how the termination message should be populated. File will use the contents of
    /// terminationMessagePath to populate the container status message on both success and
    /// failure. FallbackToLogsOnError will use the last chunk of container log output if the
    /// termination message file is empty and the container exited with an error. The log output
    /// is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be
    /// updated.
    #[serde(rename = "terminationMessagePolicy")]
    termination_message_policy: Option<String>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be
    /// true. Default is false.
    tty: Option<bool>,
    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[serde(rename = "volumeMounts")]
    volume_mounts: Option<Vec<Option<ContainerVolumeMount>>>,
    /// Container's working directory. If not specified, the container runtime's default will be
    /// used, which might be configured in the container image. Cannot be updated.
    #[serde(rename = "workingDir")]
    working_dir: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    name: String,
    /// Variable references $(VAR_NAME) are expanded using the previous defined environment
    /// variables in the container and any service environment variables. If a variable cannot be
    /// resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
    /// be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
    /// regardless of whether the variable exists or not. Defaults to "".
    value: Option<String>,
    /// EnvVarSource represents a source for the value of an EnvVar.
    #[serde(rename = "valueFrom")]
    value_from: Option<TentacledValueFrom>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleValueFrom {
    /// Selects a key from a ConfigMap.
    #[serde(rename = "configMapKeyRef")]
    config_map_key_ref: Option<TentacledConfigMapKeyRef>,
    /// ObjectFieldSelector selects an APIVersioned field of an object.
    #[serde(rename = "fieldRef")]
    field_ref: Option<IndigoFieldRef>,
    /// ResourceFieldSelector represents container resources (cpu, memory) and their output format
    #[serde(rename = "resourceFieldRef")]
    resource_field_ref: Option<IndigoResourceFieldRef>,
    /// SecretKeySelector selects a key of a Secret.
    #[serde(rename = "secretKeyRef")]
    secret_key_ref: Option<TentacledSecretKeyRef>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleConfigMapKeyRef {
    /// The key to select.
    key: String,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the ConfigMap or it's key must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(rename = "apiVersion")]
    api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    field_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(rename = "containerName")]
    container_name: Option<String>,
    divisor: Option<MaxSurge>,
    /// Required: resource to select
    resource: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    key: String,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the Secret or it's key must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleEnvFrom {
    /// ConfigMapEnvSource selects a ConfigMap to populate the environment variables with.
    ///
    /// The contents of the target ConfigMap's Data field will represent the key-value pairs as
    /// environment variables.
    #[serde(rename = "configMapRef")]
    config_map_ref: Option<TentacledConfigMapRef>,
    /// An optional identifer to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    prefix: Option<String>,
    /// SecretEnvSource selects a Secret to populate the environment variables with.
    ///
    /// The contents of the target Secret's Data field will represent the key-value pairs as
    /// environment variables.
    #[serde(rename = "secretRef")]
    secret_ref: Option<HilariousSecretRef>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleConfigMapRef {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the ConfigMap must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSecretRef {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the Secret must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleLifecycle {
    /// Handler defines a specific action that should be taken
    #[serde(rename = "postStart")]
    post_start: Option<TentacledPostStart>,
    /// Handler defines a specific action that should be taken
    #[serde(rename = "preStop")]
    pre_stop: Option<TentacledPreStop>,
}

#[derive(Serialize, Deserialize)]
pub struct PurplePostStart {
    /// ExecAction describes a "run in container" action.
    exec: Option<CunningExec>,
    /// HTTPGetAction describes an action based on HTTP Get requests.
    #[serde(rename = "httpGet")]
    http_get: Option<CunningHttpGet>,
    /// TCPSocketAction describes an action based on opening a socket
    #[serde(rename = "tcpSocket")]
    tcp_socket: Option<CunningTcpSocket>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleExec {
    /// Command is the command line to execute inside the container, the working directory for
    /// the command  is root ('/') in the container's filesystem. The command is simply exec'd,
    /// it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To
    /// use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated
    /// as live/healthy and non-zero is unhealthy.
    command: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in
    /// httpHeaders instead.
    host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    http_headers: Option<Vec<Option<CunningHttpHeader>>>,
    /// Path to access on the HTTP server.
    path: Option<String>,
    port: Option<MaxSurge>,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    scheme: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleHttpHeader {
    /// The header field name
    name: String,
    /// The header field value
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleTcpSocket {
    port: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct PurplePreStop {
    /// ExecAction describes a "run in container" action.
    exec: Option<MagentaExec>,
    /// HTTPGetAction describes an action based on HTTP Get requests.
    #[serde(rename = "httpGet")]
    http_get: Option<MagentaHttpGet>,
    /// TCPSocketAction describes an action based on opening a socket
    #[serde(rename = "tcpSocket")]
    tcp_socket: Option<MagentaTcpSocket>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyExec {
    /// Command is the command line to execute inside the container, the working directory for
    /// the command  is root ('/') in the container's filesystem. The command is simply exec'd,
    /// it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To
    /// use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated
    /// as live/healthy and non-zero is unhealthy.
    command: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in
    /// httpHeaders instead.
    host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    http_headers: Option<Vec<Option<MagentaHttpHeader>>>,
    /// Path to access on the HTTP server.
    path: Option<String>,
    port: Option<MaxSurge>,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    scheme: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyHttpHeader {
    /// The header field name
    name: String,
    /// The header field value
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyTcpSocket {
    port: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleLivenessProbe {
    /// ExecAction describes a "run in container" action.
    exec: Option<FriskyExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having
    /// succeeded. Defaults to 3. Minimum value is 1.
    #[serde(rename = "failureThreshold")]
    failure_threshold: Option<i64>,
    /// HTTPGetAction describes an action based on HTTP Get requests.
    #[serde(rename = "httpGet")]
    http_get: Option<FriskyHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated.
    /// More info: http://kubernetes.io/docs/user-guide/pod-states#container-probes
    #[serde(rename = "initialDelaySeconds")]
    initial_delay_seconds: Option<i64>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(rename = "periodSeconds")]
    period_seconds: Option<i64>,
    /// Minimum consecutive successes for the probe to be considered successful after having
    /// failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1.
    #[serde(rename = "successThreshold")]
    success_threshold: Option<i64>,
    /// TCPSocketAction describes an action based on opening a socket
    #[serde(rename = "tcpSocket")]
    tcp_socket: Option<FriskyTcpSocket>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is
    /// 1. More info: http://kubernetes.io/docs/user-guide/pod-states#container-probes
    #[serde(rename = "timeoutSeconds")]
    timeout_seconds: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledExec {
    /// Command is the command line to execute inside the container, the working directory for
    /// the command  is root ('/') in the container's filesystem. The command is simply exec'd,
    /// it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To
    /// use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated
    /// as live/healthy and non-zero is unhealthy.
    command: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in
    /// httpHeaders instead.
    host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    http_headers: Option<Vec<Option<FriskyHttpHeader>>>,
    /// Path to access on the HTTP server.
    path: Option<String>,
    port: Option<MaxSurge>,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    scheme: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledHttpHeader {
    /// The header field name
    name: String,
    /// The header field value
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledTcpSocket {
    port: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct PurplePort {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x
    /// < 65536.
    #[serde(rename = "containerPort")]
    container_port: i64,
    /// What host IP to bind the external port to.
    #[serde(rename = "hostIP")]
    host_ip: Option<String>,
    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 <
    /// x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do
    /// not need this.
    #[serde(rename = "hostPort")]
    host_port: Option<i64>,
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in
    /// a pod must have a unique name. Name for the port that can be referred to by services.
    name: Option<String>,
    /// Protocol for port. Must be UDP or TCP. Defaults to "TCP".
    protocol: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleReadinessProbe {
    /// ExecAction describes a "run in container" action.
    exec: Option<MischievousExec>,
    /// Minimum consecutive failures for the probe to be considered failed after having
    /// succeeded. Defaults to 3. Minimum value is 1.
    #[serde(rename = "failureThreshold")]
    failure_threshold: Option<i64>,
    /// HTTPGetAction describes an action based on HTTP Get requests.
    #[serde(rename = "httpGet")]
    http_get: Option<MischievousHttpGet>,
    /// Number of seconds after the container has started before liveness probes are initiated.
    /// More info: http://kubernetes.io/docs/user-guide/pod-states#container-probes
    #[serde(rename = "initialDelaySeconds")]
    initial_delay_seconds: Option<i64>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(rename = "periodSeconds")]
    period_seconds: Option<i64>,
    /// Minimum consecutive successes for the probe to be considered successful after having
    /// failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1.
    #[serde(rename = "successThreshold")]
    success_threshold: Option<i64>,
    /// TCPSocketAction describes an action based on opening a socket
    #[serde(rename = "tcpSocket")]
    tcp_socket: Option<MischievousTcpSocket>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is
    /// 1. More info: http://kubernetes.io/docs/user-guide/pod-states#container-probes
    #[serde(rename = "timeoutSeconds")]
    timeout_seconds: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyExec {
    /// Command is the command line to execute inside the container, the working directory for
    /// the command  is root ('/') in the container's filesystem. The command is simply exec'd,
    /// it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To
    /// use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated
    /// as live/healthy and non-zero is unhealthy.
    command: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in
    /// httpHeaders instead.
    host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    http_headers: Option<Vec<Option<MischievousHttpHeader>>>,
    /// Path to access on the HTTP server.
    path: Option<String>,
    port: Option<MaxSurge>,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    scheme: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyHttpHeader {
    /// The header field name
    name: String,
    /// The header field value
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct StickyTcpSocket {
    port: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleResources {
    /// Limits describes the maximum amount of compute resources allowed. More info:
    /// http://kubernetes.io/docs/user-guide/compute-resources/
    limits: Option<HashMap<String, Option<MaxSurge>>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is
    /// omitted for a container, it defaults to Limits if that is explicitly specified, otherwise
    /// to an implementation-defined value. More info:
    /// http://kubernetes.io/docs/user-guide/compute-resources/
    requests: Option<HashMap<String, Option<MaxSurge>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSecurityContext {
    /// Adds and removes POSIX capabilities from running containers.
    capabilities: Option<TentacledCapabilities>,
    /// Run container in privileged mode. Processes in privileged containers are essentially
    /// equivalent to root on the host. Defaults to false.
    privileged: Option<bool>,
    /// Whether this container has a read-only root filesystem. Default is false.
    #[serde(rename = "readOnlyRootFilesystem")]
    read_only_root_filesystem: Option<bool>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will
    /// validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to
    /// start the container if it does. If unset or false, no such validation will be performed.
    /// May also be set in PodSecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "runAsNonRoot")]
    run_as_non_root: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in
    /// image metadata if unspecified. May also be set in PodSecurityContext.  If set in both
    /// SecurityContext and PodSecurityContext, the value specified in SecurityContext takes
    /// precedence.
    #[serde(rename = "runAsUser")]
    run_as_user: Option<i64>,
    /// SELinuxOptions are the labels to be applied to the container
    #[serde(rename = "seLinuxOptions")]
    se_linux_options: Option<StickySeLinuxOptions>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleCapabilities {
    /// Added capabilities
    add: Option<Vec<Option<String>>>,
    /// Removed capabilities
    drop: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSeLinuxOptions {
    /// Level is SELinux level label that applies to the container.
    level: Option<String>,
    /// Role is a SELinux role label that applies to the container.
    role: Option<String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(rename = "type")]
    se_linux_options_type: Option<String>,
    /// User is a SELinux user label that applies to the container.
    user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleVolumeMount {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    #[serde(rename = "mountPath")]
    mount_path: String,
    /// This must match the Name of a Volume.
    name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to
    /// "" (volume's root).
    #[serde(rename = "subPath")]
    sub_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ImagePullSecretClass {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InitContainerClass {
    /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided.
    /// Variable references $(VAR_NAME) are expanded using the container's environment. If a
    /// variable cannot be resolved, the reference in the input string will be unchanged. The
    /// $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references
    /// will never be expanded, regardless of whether the variable exists or not. Cannot be
    /// updated. More info:
    /// http://kubernetes.io/docs/user-guide/containers#containers-and-commands
    args: Option<Vec<Option<String>>>,
    /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if
    /// this is not provided. Variable references $(VAR_NAME) are expanded using the container's
    /// environment. If a variable cannot be resolved, the reference in the input string will be
    /// unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME).
    /// Escaped references will never be expanded, regardless of whether the variable exists or
    /// not. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/containers#containers-and-commands
    command: Option<Vec<Option<String>>>,
    /// List of environment variables to set in the container. Cannot be updated.
    env: Option<Vec<Option<InitContainerEnv>>>,
    /// List of sources to populate environment variables in the container. The keys defined
    /// within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event
    /// when the container is starting. When a key exists in multiple sources, the value
    /// associated with the last source will take precedence. Values defined by an Env with a
    /// duplicate key will take precedence. Cannot be updated.
    #[serde(rename = "envFrom")]
    env_from: Option<Vec<Option<InitContainerEnvFrom>>>,
    /// Docker image name. More info: http://kubernetes.io/docs/user-guide/images
    image: Option<String>,
    /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag
    /// is specified, or IfNotPresent otherwise. Cannot be updated. More info:
    /// http://kubernetes.io/docs/user-guide/images#updating-images
    #[serde(rename = "imagePullPolicy")]
    image_pull_policy: Option<String>,
    /// Lifecycle describes actions that the management system should take in response to
    /// container lifecycle events. For the PostStart and PreStop lifecycle handlers, management
    /// of the container blocks until the action is complete, unless the container process fails,
    /// in which case the handler is aborted.
    lifecycle: Option<InitContainerLifecycle>,
    /// Probe describes a health check to be performed against a container to determine whether
    /// it is alive or ready to receive traffic.
    #[serde(rename = "livenessProbe")]
    liveness_probe: Option<InitContainerLivenessProbe>,
    /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a
    /// unique name (DNS_LABEL). Cannot be updated.
    name: String,
    /// List of ports to expose from the container. Exposing a port here gives the system
    /// additional information about the network connections a container uses, but is primarily
    /// informational. Not specifying a port here DOES NOT prevent that port from being exposed.
    /// Any port which is listening on the default "0.0.0.0" address inside a container will be
    /// accessible from the network. Cannot be updated.
    ports: Option<Vec<Option<InitContainerPort>>>,
    /// Probe describes a health check to be performed against a container to determine whether
    /// it is alive or ready to receive traffic.
    #[serde(rename = "readinessProbe")]
    readiness_probe: Option<InitContainerReadinessProbe>,
    /// ResourceRequirements describes the compute resource requirements.
    resources: Option<InitContainerResources>,
    /// SecurityContext holds security configuration that will be applied to a container. Some
    /// fields are present in both SecurityContext and PodSecurityContext.  When both are set,
    /// the values in SecurityContext take precedence.
    #[serde(rename = "securityContext")]
    security_context: Option<InitContainerSecurityContext>,
    /// Whether this container should allocate a buffer for stdin in the container runtime. If
    /// this is not set, reads from stdin in the container will always result in EOF. Default is
    /// false.
    stdin: Option<bool>,
    /// Whether the container runtime should close the stdin channel after it has been opened by
    /// a single attach. When stdin is true the stdin stream will remain open across multiple
    /// attach sessions. If stdinOnce is set to true, stdin is opened on container start, is
    /// empty until the first client attaches to stdin, and then remains open and accepts data
    /// until the client disconnects, at which time stdin is closed and remains closed until the
    /// container is restarted. If this flag is false, a container processes that reads from
    /// stdin will never receive an EOF. Default is false
    #[serde(rename = "stdinOnce")]
    stdin_once: Option<bool>,
    /// Optional: Path at which the file to which the container's termination message will be
    /// written is mounted into the container's filesystem. Message written is intended to be
    /// brief final status, such as an assertion failure message. Will be truncated by the node
    /// if greater than 4096 bytes. The total message length across all containers will be
    /// limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
    #[serde(rename = "terminationMessagePath")]
    termination_message_path: Option<String>,
    /// Indicate how the termination message should be populated. File will use the contents of
    /// terminationMessagePath to populate the container status message on both success and
    /// failure. FallbackToLogsOnError will use the last chunk of container log output if the
    /// termination message file is empty and the container exited with an error. The log output
    /// is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be
    /// updated.
    #[serde(rename = "terminationMessagePolicy")]
    termination_message_policy: Option<String>,
    /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be
    /// true. Default is false.
    tty: Option<bool>,
    /// Pod volumes to mount into the container's filesystem. Cannot be updated.
    #[serde(rename = "volumeMounts")]
    volume_mounts: Option<Vec<Option<InitContainerVolumeMount>>>,
    /// Container's working directory. If not specified, the container runtime's default will be
    /// used, which might be configured in the container image. Cannot be updated.
    #[serde(rename = "workingDir")]
    working_dir: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyEnv {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    name: String,
    /// Variable references $(VAR_NAME) are expanded using the previous defined environment
    /// variables in the container and any service environment variables. If a variable cannot be
    /// resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can
    /// be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
    /// regardless of whether the variable exists or not. Defaults to "".
    value: Option<String>,
    /// EnvVarSource represents a source for the value of an EnvVar.
    #[serde(rename = "valueFrom")]
    value_from: Option<StickyValueFrom>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyValueFrom {
    /// Selects a key from a ConfigMap.
    #[serde(rename = "configMapKeyRef")]
    config_map_key_ref: Option<StickyConfigMapKeyRef>,
    /// ObjectFieldSelector selects an APIVersioned field of an object.
    #[serde(rename = "fieldRef")]
    field_ref: Option<IndecentFieldRef>,
    /// ResourceFieldSelector represents container resources (cpu, memory) and their output format
    #[serde(rename = "resourceFieldRef")]
    resource_field_ref: Option<IndecentResourceFieldRef>,
    /// SecretKeySelector selects a key of a Secret.
    #[serde(rename = "secretKeyRef")]
    secret_key_ref: Option<StickySecretKeyRef>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyConfigMapKeyRef {
    /// The key to select.
    key: String,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the ConfigMap or it's key must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(rename = "apiVersion")]
    api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    field_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(rename = "containerName")]
    container_name: Option<String>,
    divisor: Option<MaxSurge>,
    /// Required: resource to select
    resource: String,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    key: String,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the Secret or it's key must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyEnvFrom {
    /// ConfigMapEnvSource selects a ConfigMap to populate the environment variables with.
    ///
    /// The contents of the target ConfigMap's Data field will represent the key-value pairs as
    /// environment variables.
    #[serde(rename = "configMapRef")]
    config_map_ref: Option<StickyConfigMapRef>,
    /// An optional identifer to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
    prefix: Option<String>,
    /// SecretEnvSource selects a Secret to populate the environment variables with.
    ///
    /// The contents of the target Secret's Data field will represent the key-value pairs as
    /// environment variables.
    #[serde(rename = "secretRef")]
    secret_ref: Option<AmbitiousSecretRef>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyConfigMapRef {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the ConfigMap must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySecretRef {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the Secret must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyLifecycle {
    /// Handler defines a specific action that should be taken
    #[serde(rename = "postStart")]
    post_start: Option<StickyPostStart>,
    /// Handler defines a specific action that should be taken
    #[serde(rename = "preStop")]
    pre_stop: Option<StickyPreStop>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyPostStart {
    /// ExecAction describes a "run in container" action.
    exec: Option<BraggadociousExec>,
    /// HTTPGetAction describes an action based on HTTP Get requests.
    #[serde(rename = "httpGet")]
    http_get: Option<BraggadociousHttpGet>,
    /// TCPSocketAction describes an action based on opening a socket
    #[serde(rename = "tcpSocket")]
    tcp_socket: Option<BraggadociousTcpSocket>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoExec {
    /// Command is the command line to execute inside the container, the working directory for
    /// the command  is root ('/') in the container's filesystem. The command is simply exec'd,
    /// it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To
    /// use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated
    /// as live/healthy and non-zero is unhealthy.
    command: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in
    /// httpHeaders instead.
    host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    http_headers: Option<Vec<Option<BraggadociousHttpHeader>>>,
    /// Path to access on the HTTP server.
    path: Option<String>,
    port: Option<MaxSurge>,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    scheme: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoHttpHeader {
    /// The header field name
    name: String,
    /// The header field value
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoTcpSocket {
    port: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyPreStop {
    /// ExecAction describes a "run in container" action.
    exec: Option<Exec1>,
    /// HTTPGetAction describes an action based on HTTP Get requests.
    #[serde(rename = "httpGet")]
    http_get: Option<HttpGet1>,
    /// TCPSocketAction describes an action based on opening a socket
    #[serde(rename = "tcpSocket")]
    tcp_socket: Option<TcpSocket1>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentExec {
    /// Command is the command line to execute inside the container, the working directory for
    /// the command  is root ('/') in the container's filesystem. The command is simply exec'd,
    /// it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To
    /// use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated
    /// as live/healthy and non-zero is unhealthy.
    command: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in
    /// httpHeaders instead.
    host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    http_headers: Option<Vec<Option<HttpHeader1>>>,
    /// Path to access on the HTTP server.
    path: Option<String>,
    port: Option<MaxSurge>,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    scheme: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentHttpHeader {
    /// The header field name
    name: String,
    /// The header field value
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentTcpSocket {
    port: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyLivenessProbe {
    /// ExecAction describes a "run in container" action.
    exec: Option<Exec2>,
    /// Minimum consecutive failures for the probe to be considered failed after having
    /// succeeded. Defaults to 3. Minimum value is 1.
    #[serde(rename = "failureThreshold")]
    failure_threshold: Option<i64>,
    /// HTTPGetAction describes an action based on HTTP Get requests.
    #[serde(rename = "httpGet")]
    http_get: Option<HttpGet2>,
    /// Number of seconds after the container has started before liveness probes are initiated.
    /// More info: http://kubernetes.io/docs/user-guide/pod-states#container-probes
    #[serde(rename = "initialDelaySeconds")]
    initial_delay_seconds: Option<i64>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(rename = "periodSeconds")]
    period_seconds: Option<i64>,
    /// Minimum consecutive successes for the probe to be considered successful after having
    /// failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1.
    #[serde(rename = "successThreshold")]
    success_threshold: Option<i64>,
    /// TCPSocketAction describes an action based on opening a socket
    #[serde(rename = "tcpSocket")]
    tcp_socket: Option<TcpSocket2>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is
    /// 1. More info: http://kubernetes.io/docs/user-guide/pod-states#container-probes
    #[serde(rename = "timeoutSeconds")]
    timeout_seconds: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousExec {
    /// Command is the command line to execute inside the container, the working directory for
    /// the command  is root ('/') in the container's filesystem. The command is simply exec'd,
    /// it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To
    /// use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated
    /// as live/healthy and non-zero is unhealthy.
    command: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in
    /// httpHeaders instead.
    host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    http_headers: Option<Vec<Option<HttpHeader2>>>,
    /// Path to access on the HTTP server.
    path: Option<String>,
    port: Option<MaxSurge>,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    scheme: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousHttpHeader {
    /// The header field name
    name: String,
    /// The header field value
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousTcpSocket {
    port: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyPort {
    /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x
    /// < 65536.
    #[serde(rename = "containerPort")]
    container_port: i64,
    /// What host IP to bind the external port to.
    #[serde(rename = "hostIP")]
    host_ip: Option<String>,
    /// Number of port to expose on the host. If specified, this must be a valid port number, 0 <
    /// x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do
    /// not need this.
    #[serde(rename = "hostPort")]
    host_port: Option<i64>,
    /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in
    /// a pod must have a unique name. Name for the port that can be referred to by services.
    name: Option<String>,
    /// Protocol for port. Must be UDP or TCP. Defaults to "TCP".
    protocol: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyReadinessProbe {
    /// ExecAction describes a "run in container" action.
    exec: Option<Exec3>,
    /// Minimum consecutive failures for the probe to be considered failed after having
    /// succeeded. Defaults to 3. Minimum value is 1.
    #[serde(rename = "failureThreshold")]
    failure_threshold: Option<i64>,
    /// HTTPGetAction describes an action based on HTTP Get requests.
    #[serde(rename = "httpGet")]
    http_get: Option<HttpGet3>,
    /// Number of seconds after the container has started before liveness probes are initiated.
    /// More info: http://kubernetes.io/docs/user-guide/pod-states#container-probes
    #[serde(rename = "initialDelaySeconds")]
    initial_delay_seconds: Option<i64>,
    /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
    #[serde(rename = "periodSeconds")]
    period_seconds: Option<i64>,
    /// Minimum consecutive successes for the probe to be considered successful after having
    /// failed. Defaults to 1. Must be 1 for liveness. Minimum value is 1.
    #[serde(rename = "successThreshold")]
    success_threshold: Option<i64>,
    /// TCPSocketAction describes an action based on opening a socket
    #[serde(rename = "tcpSocket")]
    tcp_socket: Option<TcpSocket3>,
    /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is
    /// 1. More info: http://kubernetes.io/docs/user-guide/pod-states#container-probes
    #[serde(rename = "timeoutSeconds")]
    timeout_seconds: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousExec {
    /// Command is the command line to execute inside the container, the working directory for
    /// the command  is root ('/') in the container's filesystem. The command is simply exec'd,
    /// it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To
    /// use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated
    /// as live/healthy and non-zero is unhealthy.
    command: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousHttpGet {
    /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in
    /// httpHeaders instead.
    host: Option<String>,
    /// Custom headers to set in the request. HTTP allows repeated headers.
    #[serde(rename = "httpHeaders")]
    http_headers: Option<Vec<Option<HttpHeader3>>>,
    /// Path to access on the HTTP server.
    path: Option<String>,
    port: Option<MaxSurge>,
    /// Scheme to use for connecting to the host. Defaults to HTTP.
    scheme: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousHttpHeader {
    /// The header field name
    name: String,
    /// The header field value
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct AmbitiousTcpSocket {
    port: Option<MaxSurge>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyResources {
    /// Limits describes the maximum amount of compute resources allowed. More info:
    /// http://kubernetes.io/docs/user-guide/compute-resources/
    limits: Option<HashMap<String, Option<MaxSurge>>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is
    /// omitted for a container, it defaults to Limits if that is explicitly specified, otherwise
    /// to an implementation-defined value. More info:
    /// http://kubernetes.io/docs/user-guide/compute-resources/
    requests: Option<HashMap<String, Option<MaxSurge>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySecurityContext {
    /// Adds and removes POSIX capabilities from running containers.
    capabilities: Option<StickyCapabilities>,
    /// Run container in privileged mode. Processes in privileged containers are essentially
    /// equivalent to root on the host. Defaults to false.
    privileged: Option<bool>,
    /// Whether this container has a read-only root filesystem. Default is false.
    #[serde(rename = "readOnlyRootFilesystem")]
    read_only_root_filesystem: Option<bool>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will
    /// validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to
    /// start the container if it does. If unset or false, no such validation will be performed.
    /// May also be set in PodSecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "runAsNonRoot")]
    run_as_non_root: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in
    /// image metadata if unspecified. May also be set in PodSecurityContext.  If set in both
    /// SecurityContext and PodSecurityContext, the value specified in SecurityContext takes
    /// precedence.
    #[serde(rename = "runAsUser")]
    run_as_user: Option<i64>,
    /// SELinuxOptions are the labels to be applied to the container
    #[serde(rename = "seLinuxOptions")]
    se_linux_options: Option<IndigoSeLinuxOptions>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyCapabilities {
    /// Added capabilities
    add: Option<Vec<Option<String>>>,
    /// Removed capabilities
    drop: Option<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySeLinuxOptions {
    /// Level is SELinux level label that applies to the container.
    level: Option<String>,
    /// Role is a SELinux role label that applies to the container.
    role: Option<String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(rename = "type")]
    se_linux_options_type: Option<String>,
    /// User is a SELinux user label that applies to the container.
    user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyVolumeMount {
    /// Path within the container at which the volume should be mounted.  Must not contain ':'.
    #[serde(rename = "mountPath")]
    mount_path: String,
    /// This must match the Name of a Volume.
    name: String,
    /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// Path within the volume from which the container's volume should be mounted. Defaults to
    /// "" (volume's root).
    #[serde(rename = "subPath")]
    sub_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledSecurityContext {
    /// A special supplemental group that applies to all containers in a pod. Some volume types
    /// allow the Kubelet to change the ownership of that volume to be owned by the pod:
    ///
    /// 1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the
    /// volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----
    ///
    /// If unset, the Kubelet will not modify the ownership and permissions of any volume.
    #[serde(rename = "fsGroup")]
    fs_group: Option<i64>,
    /// Indicates that the container must run as a non-root user. If true, the Kubelet will
    /// validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to
    /// start the container if it does. If unset or false, no such validation will be performed.
    /// May also be set in SecurityContext.  If set in both SecurityContext and
    /// PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(rename = "runAsNonRoot")]
    run_as_non_root: Option<bool>,
    /// The UID to run the entrypoint of the container process. Defaults to user specified in
    /// image metadata if unspecified. May also be set in SecurityContext.  If set in both
    /// SecurityContext and PodSecurityContext, the value specified in SecurityContext takes
    /// precedence for that container.
    #[serde(rename = "runAsUser")]
    run_as_user: Option<i64>,
    /// SELinuxOptions are the labels to be applied to the container
    #[serde(rename = "seLinuxOptions")]
    se_linux_options: Option<IndecentSeLinuxOptions>,
    /// A list of groups applied to the first process run in each container, in addition to the
    /// container's primary GID.  If unspecified, no groups will be added to any container.
    #[serde(rename = "supplementalGroups")]
    supplemental_groups: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledSeLinuxOptions {
    /// Level is SELinux level label that applies to the container.
    level: Option<String>,
    /// Role is a SELinux role label that applies to the container.
    role: Option<String>,
    /// Type is a SELinux type label that applies to the container.
    #[serde(rename = "type")]
    se_linux_options_type: Option<String>,
    /// User is a SELinux user label that applies to the container.
    user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TolerationClass {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When
    /// specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If
    /// the key is empty, operator must be Exists; this combination means to match all values and
    /// all keys.
    key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and
    /// Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can
    /// tolerate all taints of a particular category.
    operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect
    /// NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not
    /// set, which means tolerate the taint forever (do not evict). Zero and negative values will
    /// be treated as 0 (evict immediately) by the system.
    #[serde(rename = "tolerationSeconds")]
    toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value
    /// should be empty, otherwise just a regular string.
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VolumeClass {
    /// Represents a Persistent Disk resource in AWS.
    ///
    /// An AWS EBS disk must exist before mounting to a container. The disk must also be in the
    /// same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS
    /// EBS volumes support ownership management and SELinux relabeling.
    #[serde(rename = "awsElasticBlockStore")]
    aws_elastic_block_store: Option<AwsElasticBlockStoreUnion>,
    /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
    #[serde(rename = "azureDisk")]
    azure_disk: Option<AzureDiskUnion>,
    /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
    #[serde(rename = "azureFile")]
    azure_file: Option<AzureFileUnion>,
    /// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not
    /// support ownership management or SELinux relabeling.
    cephfs: Option<CephfsUnion>,
    /// Represents a cinder volume resource in Openstack. A Cinder volume must exist before
    /// mounting to a container. The volume must also be in the same region as the kubelet.
    /// Cinder volumes support ownership management and SELinux relabeling.
    cinder: Option<CinderUnion>,
    /// Adapts a ConfigMap into a volume.
    ///
    /// The contents of the target ConfigMap's Data field will be presented in a volume as files
    /// using the keys in the Data field as the file names, unless the items element is populated
    /// with specific mappings of keys to paths. ConfigMap volumes support ownership management
    /// and SELinux relabeling.
    #[serde(rename = "configMap")]
    config_map: Option<VolumeConfigMap>,
    /// DownwardAPIVolumeSource represents a volume containing downward API info. Downward API
    /// volumes support ownership management and SELinux relabeling.
    #[serde(rename = "downwardAPI")]
    downward_api: Option<VolumeDownwardApi>,
    /// Represents an empty directory for a pod. Empty directory volumes support ownership
    /// management and SELinux relabeling.
    #[serde(rename = "emptyDir")]
    empty_dir: Option<EmptyDirUnion>,
    /// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as
    /// read/write once. Fibre Channel volumes support ownership management and SELinux
    /// relabeling.
    fc: Option<FcUnion>,
    /// FlexVolume represents a generic volume resource that is provisioned/attached using an
    /// exec based plugin. This is an alpha feature and may change in future.
    #[serde(rename = "flexVolume")]
    flex_volume: Option<FlexVolumeUnion>,
    /// Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName
    /// and datasetUUID should be set. Flocker volumes do not support ownership management or
    /// SELinux relabeling.
    flocker: Option<FlockerUnion>,
    /// Represents a Persistent Disk resource in Google Compute Engine.
    ///
    /// A GCE PD must exist before mounting to a container. The disk must also be in the same GCE
    /// project and zone as the kubelet. A GCE PD can only be mounted as read/write once or
    /// read-only many times. GCE PDs support ownership management and SELinux relabeling.
    #[serde(rename = "gcePersistentDisk")]
    gce_persistent_disk: Option<GcePersistentDiskUnion>,
    /// Represents a volume that is populated with the contents of a git repository. Git repo
    /// volumes do not support ownership management. Git repo volumes support SELinux relabeling.
    #[serde(rename = "gitRepo")]
    git_repo: Option<GitRepoUnion>,
    /// Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not
    /// support ownership management or SELinux relabeling.
    glusterfs: Option<GlusterfsUnion>,
    /// Represents a host path mapped into a pod. Host path volumes do not support ownership
    /// management or SELinux relabeling.
    #[serde(rename = "hostPath")]
    host_path: Option<HostPathUnion>,
    /// Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI
    /// volumes support ownership management and SELinux relabeling.
    iscsi: Option<IscsiUnion>,
    /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info:
    /// http://kubernetes.io/docs/user-guide/identifiers#names
    name: String,
    /// Represents an NFS mount that lasts the lifetime of a pod. NFS volumes do not support
    /// ownership management or SELinux relabeling.
    nfs: Option<NfsUnion>,
    /// PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This
    /// volume finds the bound PV and mounts that volume for the pod. A
    /// PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of
    /// volume that is owned by someone else (the system).
    #[serde(rename = "persistentVolumeClaim")]
    persistent_volume_claim: Option<PersistentVolumeClaimUnion>,
    /// Represents a Photon Controller persistent disk resource.
    #[serde(rename = "photonPersistentDisk")]
    photon_persistent_disk: Option<PhotonPersistentDiskUnion>,
    /// PortworxVolumeSource represents a Portworx volume resource.
    #[serde(rename = "portworxVolume")]
    portworx_volume: Option<PortworxVolumeUnion>,
    /// Represents a projected volume source
    projected: Option<ProjectedUnion>,
    /// Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not
    /// support ownership management or SELinux relabeling.
    quobyte: Option<QuobyteUnion>,
    /// Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes
    /// support ownership management and SELinux relabeling.
    rbd: Option<RbdUnion>,
    /// ScaleIOVolumeSource represents a persistent ScaleIO volume
    #[serde(rename = "scaleIO")]
    scale_io: Option<ScaleIoUnion>,
    /// Adapts a Secret into a volume.
    ///
    /// The contents of the target Secret's Data field will be presented in a volume as files
    /// using the keys in the Data field as the file names. Secret volumes support ownership
    /// management and SELinux relabeling.
    secret: Option<VolumeSecret>,
    /// Represents a vSphere volume resource.
    #[serde(rename = "vsphereVolume")]
    vsphere_volume: Option<VsphereVolumeUnion>,
}

#[derive(Serialize, Deserialize)]
pub struct AwsElasticBlockStoreClass {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem
    /// type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs".
    /// Implicitly inferred to be "ext4" if unspecified. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#awselasticblockstore
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// The partition in the volume that you want to mount. If omitted, the default is to mount
    /// by volume name. Examples: For volume /dev/sda1, you specify the partition as "1".
    /// Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
    partition: Option<i64>,
    /// Specify "true" to force and set the ReadOnly property in VolumeMounts to "true". If
    /// omitted, the default is "false". More info:
    /// http://kubernetes.io/docs/user-guide/volumes#awselasticblockstore
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info:
    /// http://kubernetes.io/docs/user-guide/volumes#awselasticblockstore
    #[serde(rename = "volumeID")]
    volume_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AzureDiskClass {
    /// Host Caching mode: None, Read Only, Read Write.
    #[serde(rename = "cachingMode")]
    caching_mode: Option<String>,
    /// The Name of the data disk in the blob storage
    #[serde(rename = "diskName")]
    disk_name: String,
    /// The URI the data disk in the blob storage
    #[serde(rename = "diskURI")]
    disk_uri: String,
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating
    /// system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in
    /// VolumeMounts.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct AzureFileClass {
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in
    /// VolumeMounts.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// the name of secret that contains Azure Storage Account Name and Key
    #[serde(rename = "secretName")]
    secret_name: String,
    /// Share Name
    #[serde(rename = "shareName")]
    share_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CephfsClass {
    /// Required: Monitors is a collection of Ceph monitors More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    monitors: Vec<Option<String>>,
    /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
    path: Option<String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting
    /// in VolumeMounts. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret
    /// More info: http://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    #[serde(rename = "secretFile")]
    secret_file: Option<String>,
    /// LocalObjectReference contains enough information to let you locate the referenced object
    /// inside the same namespace.
    #[serde(rename = "secretRef")]
    secret_ref: Option<CephfsSecretRef>,
    /// Optional: User is the rados user name, default is admin More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/cephfs/README.md#how-to-use-it
    user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledSecretRef {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CinderClass {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating
    /// system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    /// More info: http://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting
    /// in VolumeMounts. More info: http://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// volume id used to identify the volume in cinder More info:
    /// http://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
    #[serde(rename = "volumeID")]
    volume_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleConfigMap {
    /// Optional: mode bits to use on created files by default. Must be a value between 0 and
    /// 0777. Defaults to 0644. Directories within the path are not affected by this setting.
    /// This might be in conflict with other options that affect the file mode, like fsGroup, and
    /// the result can be other mode bits set.
    #[serde(rename = "defaultMode")]
    default_mode: Option<i64>,
    /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be
    /// projected into the volume as a file whose name is the key and content is the value. If
    /// specified, the listed keys will be projected into the specified paths, and unlisted keys
    /// will not be present. If a key is specified which is not present in the ConfigMap, the
    /// volume setup will error unless it is marked optional. Paths must be relative and may not
    /// contain the '..' path or start with '..'.
    items: Option<Vec<Option<HilariousItem>>>,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the ConfigMap or it's keys must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleItem {
    /// The key to project.
    key: String,
    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not
    /// specified, the volume defaultMode will be used. This might be in conflict with other
    /// options that affect the file mode, like fsGroup, and the result can be other mode bits
    /// set.
    mode: Option<i64>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not
    /// contain the path element '..'. May not start with the string '..'.
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleDownwardApi {
    /// Optional: mode bits to use on created files by default. Must be a value between 0 and
    /// 0777. Defaults to 0644. Directories within the path are not affected by this setting.
    /// This might be in conflict with other options that affect the file mode, like fsGroup, and
    /// the result can be other mode bits set.
    #[serde(rename = "defaultMode")]
    default_mode: Option<i64>,
    /// Items is a list of downward API volume file
    items: Option<Vec<Option<AmbitiousItem>>>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyItem {
    /// ObjectFieldSelector selects an APIVersioned field of an object.
    #[serde(rename = "fieldRef")]
    field_ref: Option<HilariousFieldRef>,
    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not
    /// specified, the volume defaultMode will be used. This might be in conflict with other
    /// options that affect the file mode, like fsGroup, and the result can be other mode bits
    /// set.
    mode: Option<i64>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute
    /// or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must
    /// not start with '..'
    path: String,
    /// ResourceFieldSelector represents container resources (cpu, memory) and their output format
    #[serde(rename = "resourceFieldRef")]
    resource_field_ref: Option<HilariousResourceFieldRef>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(rename = "apiVersion")]
    api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    field_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(rename = "containerName")]
    container_name: Option<String>,
    divisor: Option<MaxSurge>,
    /// Required: resource to select
    resource: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmptyDirClass {
    /// What type of storage medium should back this directory. The default is "" which means to
    /// use the node's default medium. Must be an empty string (default) or Memory. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#emptydir
    medium: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FcClass {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating
    /// system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// Required: FC target lun number
    lun: i64,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting
    /// in VolumeMounts.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// Required: FC target worldwide names (WWNs)
    #[serde(rename = "targetWWNs")]
    target_ww_ns: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct FlexVolumeClass {
    /// Driver is the name of the driver to use for this volume.
    driver: String,
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating
    /// system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// Optional: Extra command options if any.
    options: Option<HashMap<String, Option<String>>>,
    /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting
    /// in VolumeMounts.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// LocalObjectReference contains enough information to let you locate the referenced object
    /// inside the same namespace.
    #[serde(rename = "secretRef")]
    secret_ref: Option<FlexVolumeSecretRef>,
}

#[derive(Serialize, Deserialize)]
pub struct StickySecretRef {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FlockerClass {
    /// Name of the dataset stored as metadata -> name on the dataset for Flocker should be
    /// considered as deprecated
    #[serde(rename = "datasetName")]
    dataset_name: Option<String>,
    /// UUID of the dataset. This is unique identifier of a Flocker dataset
    #[serde(rename = "datasetUUID")]
    dataset_uuid: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GcePersistentDiskClass {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem
    /// type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs".
    /// Implicitly inferred to be "ext4" if unspecified. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#gcepersistentdisk
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// The partition in the volume that you want to mount. If omitted, the default is to mount
    /// by volume name. Examples: For volume /dev/sda1, you specify the partition as "1".
    /// Similarly, the volume partition for /dev/sda is "0" (or you can leave the property
    /// empty). More info: http://kubernetes.io/docs/user-guide/volumes#gcepersistentdisk
    partition: Option<i64>,
    /// Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#gcepersistentdisk
    #[serde(rename = "pdName")]
    pd_name: String,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More
    /// info: http://kubernetes.io/docs/user-guide/volumes#gcepersistentdisk
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct GitRepoClass {
    /// Target directory name. Must not contain or start with '..'.  If '.' is supplied, the
    /// volume directory will be the git repository.  Otherwise, if specified, the volume will
    /// contain the git repository in the subdirectory with the given name.
    directory: Option<String>,
    /// Repository URL
    repository: String,
    /// Commit hash for the specified revision.
    revision: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GlusterfsClass {
    /// EndpointsName is the endpoint name that details Glusterfs topology. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
    endpoints: String,
    /// Path is the Glusterfs volume path. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
    path: String,
    /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions.
    /// Defaults to false. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md#create-a-pod
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct HostPathClass {
    /// Path of the directory on the host. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#hostpath
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct IscsiClass {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem
    /// type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs".
    /// Implicitly inferred to be "ext4" if unspecified. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#iscsi
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// Target iSCSI Qualified Name.
    iqn: String,
    /// Optional: Defaults to 'default' (tcp). iSCSI interface name that uses an iSCSI transport.
    #[serde(rename = "iscsiInterface")]
    iscsi_interface: Option<String>,
    /// iSCSI target lun number.
    lun: i64,
    /// iSCSI target portal List. The portal is either an IP or ip_addr:port if the port is other
    /// than default (typically TCP ports 860 and 3260).
    portals: Option<Vec<Option<String>>>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// iSCSI target portal. The portal is either an IP or ip_addr:port if the port is other than
    /// default (typically TCP ports 860 and 3260).
    #[serde(rename = "targetPortal")]
    target_portal: String,
}

#[derive(Serialize, Deserialize)]
pub struct NfsClass {
    /// Path that is exported by the NFS server. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#nfs
    path: String,
    /// ReadOnly here will force the NFS export to be mounted with read-only permissions.
    /// Defaults to false. More info: http://kubernetes.io/docs/user-guide/volumes#nfs
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// Server is the hostname or IP address of the NFS server. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#nfs
    server: String,
}

#[derive(Serialize, Deserialize)]
pub struct PersistentVolumeClaimClass {
    /// ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using
    /// this volume. More info:
    /// http://kubernetes.io/docs/user-guide/persistent-volumes#persistentvolumeclaims
    #[serde(rename = "claimName")]
    claim_name: String,
    /// Will force the ReadOnly setting in VolumeMounts. Default false.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct PhotonPersistentDiskClass {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating
    /// system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// ID that identifies Photon Controller persistent disk
    #[serde(rename = "pdID")]
    pd_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PortworxVolumeClass {
    /// FSType represents the filesystem type to mount Must be a filesystem type supported by the
    /// host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in
    /// VolumeMounts.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// VolumeID uniquely identifies a Portworx volume
    #[serde(rename = "volumeID")]
    volume_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectedClass {
    /// Mode bits to use on created files by default. Must be a value between 0 and 0777.
    /// Directories within the path are not affected by this setting. This might be in conflict
    /// with other options that affect the file mode, like fsGroup, and the result can be other
    /// mode bits set.
    #[serde(rename = "defaultMode")]
    default_mode: Option<i64>,
    /// list of volume projections
    sources: Vec<Option<SourceElement>>,
}

#[derive(Serialize, Deserialize)]
pub struct SourceClass {
    /// Adapts a ConfigMap into a projected volume.
    ///
    /// The contents of the target ConfigMap's Data field will be presented in a projected volume
    /// as files using the keys in the Data field as the file names, unless the items element is
    /// populated with specific mappings of keys to paths. Note that this is identical to a
    /// configmap volume source without the default mode.
    #[serde(rename = "configMap")]
    config_map: Option<SourceConfigMap>,
    /// Represents downward API info for projecting into a projected volume. Note that this is
    /// identical to a downwardAPI volume source without the default mode.
    #[serde(rename = "downwardAPI")]
    downward_api: Option<SourceDownwardApi>,
    /// Adapts a secret into a projected volume.
    ///
    /// The contents of the target Secret's Data field will be presented in a projected volume as
    /// files using the keys in the Data field as the file names. Note that this is identical to
    /// a secret volume source without the default mode.
    secret: Option<SourceSecret>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyConfigMap {
    /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be
    /// projected into the volume as a file whose name is the key and content is the value. If
    /// specified, the listed keys will be projected into the specified paths, and unlisted keys
    /// will not be present. If a key is specified which is not present in the ConfigMap, the
    /// volume setup will error unless it is marked optional. Paths must be relative and may not
    /// contain the '..' path or start with '..'.
    items: Option<Vec<Option<CunningItem>>>,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the ConfigMap or it's keys must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledItem {
    /// The key to project.
    key: String,
    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not
    /// specified, the volume defaultMode will be used. This might be in conflict with other
    /// options that affect the file mode, like fsGroup, and the result can be other mode bits
    /// set.
    mode: Option<i64>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not
    /// contain the path element '..'. May not start with the string '..'.
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyDownwardApi {
    /// Items is a list of DownwardAPIVolume file
    items: Option<Vec<Option<MagentaItem>>>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyItem {
    /// ObjectFieldSelector selects an APIVersioned field of an object.
    #[serde(rename = "fieldRef")]
    field_ref: Option<AmbitiousFieldRef>,
    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not
    /// specified, the volume defaultMode will be used. This might be in conflict with other
    /// options that affect the file mode, like fsGroup, and the result can be other mode bits
    /// set.
    mode: Option<i64>,
    /// Required: Path is  the relative path name of the file to be created. Must not be absolute
    /// or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must
    /// not start with '..'
    path: String,
    /// ResourceFieldSelector represents container resources (cpu, memory) and their output format
    #[serde(rename = "resourceFieldRef")]
    resource_field_ref: Option<AmbitiousResourceFieldRef>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(rename = "apiVersion")]
    api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    field_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct StickyResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(rename = "containerName")]
    container_name: Option<String>,
    divisor: Option<MaxSurge>,
    /// Required: resource to select
    resource: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSecret {
    /// If unspecified, each key-value pair in the Data field of the referenced Secret will be
    /// projected into the volume as a file whose name is the key and content is the value. If
    /// specified, the listed keys will be projected into the specified paths, and unlisted keys
    /// will not be present. If a key is specified which is not present in the Secret, the volume
    /// setup will error unless it is marked optional. Paths must be relative and may not contain
    /// the '..' path or start with '..'.
    items: Option<Vec<Option<FriskyItem>>>,
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    optional: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoItem {
    /// The key to project.
    key: String,
    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not
    /// specified, the volume defaultMode will be used. This might be in conflict with other
    /// options that affect the file mode, like fsGroup, and the result can be other mode bits
    /// set.
    mode: Option<i64>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not
    /// contain the path element '..'. May not start with the string '..'.
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuobyteClass {
    /// Group to map volume access to Default is no group
    group: Option<String>,
    /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions.
    /// Defaults to false.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// Registry represents a single or multiple Quobyte Registry services specified as a string
    /// as host:port pair (multiple entries are separated with commas) which acts as the central
    /// registry for volumes
    registry: String,
    /// User to map volume access to Defaults to serivceaccount user
    user: Option<String>,
    /// Volume is a string that references an already created Quobyte volume by name.
    volume: String,
}

#[derive(Serialize, Deserialize)]
pub struct RbdClass {
    /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem
    /// type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs".
    /// Implicitly inferred to be "ext4" if unspecified. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#rbd
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// The rados image name. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    image: String,
    /// Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    keyring: Option<String>,
    /// A collection of Ceph monitors. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    monitors: Vec<Option<String>>,
    /// The rados pool name. Default is rbd. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it.
    pool: Option<String>,
    /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More
    /// info: http://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// LocalObjectReference contains enough information to let you locate the referenced object
    /// inside the same namespace.
    #[serde(rename = "secretRef")]
    secret_ref: Option<RbdSecretRef>,
    /// The rados user name. Default is admin. More info:
    /// http://releases.k8s.io/HEAD/examples/volumes/rbd/README.md#how-to-use-it
    user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoSecretRef {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ScaleIoClass {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating
    /// system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// The host address of the ScaleIO API Gateway.
    gateway: String,
    /// The name of the Protection Domain for the configured storage (defaults to "default").
    #[serde(rename = "protectionDomain")]
    protection_domain: Option<String>,
    /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in
    /// VolumeMounts.
    #[serde(rename = "readOnly")]
    read_only: Option<bool>,
    /// LocalObjectReference contains enough information to let you locate the referenced object
    /// inside the same namespace.
    #[serde(rename = "secretRef")]
    secret_ref: Option<ScaleIoSecretRef>,
    /// Flag to enable/disable SSL communication with Gateway, default false
    #[serde(rename = "sslEnabled")]
    ssl_enabled: Option<bool>,
    /// Indicates whether the storage for a volume should be thick or thin (defaults to "thin").
    #[serde(rename = "storageMode")]
    storage_mode: Option<String>,
    /// The Storage Pool associated with the protection domain (defaults to "default").
    #[serde(rename = "storagePool")]
    storage_pool: Option<String>,
    /// The name of the storage system as configured in ScaleIO.
    system: String,
    /// The name of a volume already created in the ScaleIO system that is associated with this
    /// volume source.
    #[serde(rename = "volumeName")]
    volume_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentSecretRef {
    /// Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySecret {
    /// Optional: mode bits to use on created files by default. Must be a value between 0 and
    /// 0777. Defaults to 0644. Directories within the path are not affected by this setting.
    /// This might be in conflict with other options that affect the file mode, like fsGroup, and
    /// the result can be other mode bits set.
    #[serde(rename = "defaultMode")]
    default_mode: Option<i64>,
    /// If unspecified, each key-value pair in the Data field of the referenced Secret will be
    /// projected into the volume as a file whose name is the key and content is the value. If
    /// specified, the listed keys will be projected into the specified paths, and unlisted keys
    /// will not be present. If a key is specified which is not present in the Secret, the volume
    /// setup will error unless it is marked optional. Paths must be relative and may not contain
    /// the '..' path or start with '..'.
    items: Option<Vec<Option<MischievousItem>>>,
    /// Specify whether the Secret or it's keys must be defined
    optional: Option<bool>,
    /// Name of the secret in the pod's namespace to use. More info:
    /// http://kubernetes.io/docs/user-guide/volumes#secrets
    #[serde(rename = "secretName")]
    secret_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentItem {
    /// The key to project.
    key: String,
    /// Optional: mode bits to use on this file, must be a value between 0 and 0777. If not
    /// specified, the volume defaultMode will be used. This might be in conflict with other
    /// options that affect the file mode, like fsGroup, and the result can be other mode bits
    /// set.
    mode: Option<i64>,
    /// The relative path of the file to map the key to. May not be an absolute path. May not
    /// contain the path element '..'. May not start with the string '..'.
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct VsphereVolumeClass {
    /// Filesystem type to mount. Must be a filesystem type supported by the host operating
    /// system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(rename = "fsType")]
    fs_type: Option<String>,
    /// Path that identifies vSphere volume vmdk
    #[serde(rename = "volumePath")]
    volume_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusClass {
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this
    /// deployment.
    #[serde(rename = "availableReplicas")]
    available_replicas: Option<i64>,
    /// Represents the latest available observations of a deployment's current state.
    conditions: Option<Vec<Option<ConditionElement>>>,
    /// The generation observed by the deployment controller.
    #[serde(rename = "observedGeneration")]
    observed_generation: Option<i64>,
    /// Total number of ready pods targeted by this deployment.
    #[serde(rename = "readyReplicas")]
    ready_replicas: Option<i64>,
    /// Total number of non-terminated pods targeted by this deployment (their labels match the
    /// selector).
    replicas: Option<i64>,
    /// Total number of unavailable pods targeted by this deployment.
    #[serde(rename = "unavailableReplicas")]
    unavailable_replicas: Option<i64>,
    /// Total number of non-terminated pods targeted by this deployment that have the desired
    /// template spec.
    #[serde(rename = "updatedReplicas")]
    updated_replicas: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionClass {
    #[serde(rename = "lastTransitionTime")]
    last_transition_time: Option<String>,
    #[serde(rename = "lastUpdateTime")]
    last_update_time: Option<String>,
    /// A human readable message indicating details about the transition.
    message: Option<String>,
    /// The reason for the condition's last transition.
    reason: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    status: String,
    /// Type of deployment condition.
    #[serde(rename = "type")]
    condition_type: String,
}

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects
/// users must create.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeploymentMetadata {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleMetadata(PurpleMetadata),
    String(String),
}

/// OwnerReference contains enough information to let you identify an owning object.
/// Currently, an owning object must be in the same namespace, so there is no namespace field.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledOwnerReference {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleOwnerReference(PurpleOwnerReference),
    String(String),
}

/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeploymentSpec {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleSpec(PurpleSpec),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RollbackToUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    RollbackToClass(RollbackToClass),
    String(String),
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null label
/// selector matches no objects.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectorUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    SelectorClass(SelectorClass),
    String(String),
}

/// A label selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectorMatchExpression {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleMatchExpression(PurpleMatchExpression),
    String(String),
}

/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StrategyUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StrategyClass(StrategyClass),
    String(String),
}

/// Spec to control the desired behavior of rolling update.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RollingUpdateUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    RollingUpdateClass(RollingUpdateClass),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaxSurge {
    Integer(i64),
    String(String),
}

/// PodTemplateSpec describes the data a pod should have when created from a template
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TemplateClass(TemplateClass),
}

/// ObjectMeta is metadata that all persisted resources must have, which includes all objects
/// users must create.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateMetadata {
    Bool(bool),
    Double(f64),
    FluffyMetadata(FluffyMetadata),
    Integer(i64),
    String(String),
}

/// OwnerReference contains enough information to let you identify an owning object.
/// Currently, an owning object must be in the same namespace, so there is no namespace field.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyOwnerReference {
    Bool(bool),
    Double(f64),
    FluffyOwnerReference(FluffyOwnerReference),
    Integer(i64),
    String(String),
}

/// PodSpec is a description of a pod.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateSpec {
    Bool(bool),
    Double(f64),
    FluffySpec(FluffySpec),
    Integer(i64),
    String(String),
}

/// Affinity is a group of affinity scheduling rules.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AffinityUnion {
    AffinityClass(AffinityClass),

    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
}

/// Node affinity is a group of node affinity scheduling rules.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeAffinityUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    NodeAffinityClass(NodeAffinityClass),
    String(String),
}

/// An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's
/// a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurplePreferredDuringSchedulingIgnoredDuringExecution(
        PurplePreferredDuringSchedulingIgnoredDuringExecution,
    ),
    String(String),
}

/// A null or empty node selector term matches no objects.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PreferenceUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PreferenceClass(PreferenceClass),
    String(String),
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PreferenceMatchExpression {
    Bool(bool),
    Double(f64),
    FluffyMatchExpression(FluffyMatchExpression),
    Integer(i64),
    String(String),
}

/// A node selector represents the union of the results of one or more label queries over a
/// set of nodes; that is, it represents the OR of the selectors represented by the node
/// selector terms.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleRequiredDuringSchedulingIgnoredDuringExecution(
        PurpleRequiredDuringSchedulingIgnoredDuringExecution,
    ),
    String(String),
}

/// A null or empty node selector term matches no objects.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeSelectorTermElement {
    Bool(bool),
    Double(f64),
    Integer(i64),
    NodeSelectorTermClass(NodeSelectorTermClass),
    String(String),
}

/// A node selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeSelectorTermMatchExpression {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledMatchExpression(TentacledMatchExpression),
}

/// Pod affinity is a group of inter pod affinity scheduling rules.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PodAffinityUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PodAffinityClass(PodAffinityClass),
    String(String),
}

/// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to
/// find the most preferred node(s)
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PodAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    Bool(bool),
    Double(f64),
    FluffyPreferredDuringSchedulingIgnoredDuringExecution(
        FluffyPreferredDuringSchedulingIgnoredDuringExecution,
    ),
    Integer(i64),
    String(String),
}

/// Defines a set of pods (namely those matching the labelSelector relative to the given
/// namespace(s)) that this pod should be co-located (affinity) or not co-located
/// (anti-affinity) with, where co-located is defined as running on a node whose value of the
/// label with key <topologyKey> tches that of any node on which a pod of the set of pods is
/// running
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledPodAffinityTerm {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurplePodAffinityTerm(PurplePodAffinityTerm),
    String(String),
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null label
/// selector matches no objects.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoLabelSelector {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleLabelSelector(PurpleLabelSelector),
    String(String),
}

/// A label selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousMatchExpression {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyMatchExpression(StickyMatchExpression),
    String(String),
}

/// Defines a set of pods (namely those matching the labelSelector relative to the given
/// namespace(s)) that this pod should be co-located (affinity) or not co-located
/// (anti-affinity) with, where co-located is defined as running on a node whose value of the
/// label with key <topologyKey> tches that of any node on which a pod of the set of pods is
/// running
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PodAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    Bool(bool),
    Double(f64),
    FluffyRequiredDuringSchedulingIgnoredDuringExecution(
        FluffyRequiredDuringSchedulingIgnoredDuringExecution,
    ),
    Integer(i64),
    String(String),
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null label
/// selector matches no objects.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentLabelSelector {
    Bool(bool),
    Double(f64),
    FluffyLabelSelector(FluffyLabelSelector),
    Integer(i64),
    String(String),
}

/// A label selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningMatchExpression {
    Bool(bool),
    Double(f64),
    IndigoMatchExpression(IndigoMatchExpression),
    Integer(i64),
    String(String),
}

/// Pod anti affinity is a group of inter pod anti affinity scheduling rules.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PodAntiAffinityUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PodAntiAffinityClass(PodAntiAffinityClass),
    String(String),
}

/// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to
/// find the most preferred node(s)
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledPreferredDuringSchedulingIgnoredDuringExecution(
        TentacledPreferredDuringSchedulingIgnoredDuringExecution,
    ),
}

/// Defines a set of pods (namely those matching the labelSelector relative to the given
/// namespace(s)) that this pod should be co-located (affinity) or not co-located
/// (anti-affinity) with, where co-located is defined as running on a node whose value of the
/// label with key <topologyKey> tches that of any node on which a pod of the set of pods is
/// running
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyPodAffinityTerm {
    Bool(bool),
    Double(f64),
    FluffyPodAffinityTerm(FluffyPodAffinityTerm),
    Integer(i64),
    String(String),
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null label
/// selector matches no objects.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousLabelSelector {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledLabelSelector(TentacledLabelSelector),
}

/// A label selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaMatchExpression {
    Bool(bool),
    Double(f64),
    IndecentMatchExpression(IndecentMatchExpression),
    Integer(i64),
    String(String),
}

/// Defines a set of pods (namely those matching the labelSelector relative to the given
/// namespace(s)) that this pod should be co-located (affinity) or not co-located
/// (anti-affinity) with, where co-located is defined as running on a node whose value of the
/// label with key <topologyKey> tches that of any node on which a pod of the set of pods is
/// running
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledRequiredDuringSchedulingIgnoredDuringExecution(
        TentacledRequiredDuringSchedulingIgnoredDuringExecution,
    ),
}

/// A label selector is a label query over a set of resources. The result of matchLabels and
/// matchExpressions are ANDed. An empty label selector matches all objects. A null label
/// selector matches no objects.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousLabelSelector {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyLabelSelector(StickyLabelSelector),
    String(String),
}

/// A label selector requirement is a selector that contains values, a key, and an operator
/// that relates the key and values.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyMatchExpression {
    Bool(bool),
    Double(f64),
    HilariousMatchExpression(HilariousMatchExpression),
    Integer(i64),
    String(String),
}

/// A single application container that you want to run within a pod.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerElement {
    Bool(bool),
    ContainerClass(ContainerClass),
    Double(f64),
    Integer(i64),
    String(String),
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerEnv {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleEnv(PurpleEnv),
    String(String),
}

/// EnvVarSource represents a source for the value of an EnvVar.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledValueFrom {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleValueFrom(PurpleValueFrom),
    String(String),
}

/// Selects a key from a ConfigMap.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledConfigMapKeyRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleConfigMapKeyRef(PurpleConfigMapKeyRef),
    String(String),
}

/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoFieldRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleFieldRef(PurpleFieldRef),
    String(String),
}

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoResourceFieldRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleResourceFieldRef(PurpleResourceFieldRef),
    String(String),
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledSecretKeyRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleSecretKeyRef(PurpleSecretKeyRef),
    String(String),
}

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerEnvFrom {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleEnvFrom(PurpleEnvFrom),
    String(String),
}

/// ConfigMapEnvSource selects a ConfigMap to populate the environment variables with.
///
/// The contents of the target ConfigMap's Data field will represent the key-value pairs as
/// environment variables.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledConfigMapRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleConfigMapRef(PurpleConfigMapRef),
    String(String),
}

/// SecretEnvSource selects a Secret to populate the environment variables with.
///
/// The contents of the target Secret's Data field will represent the key-value pairs as
/// environment variables.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousSecretRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleSecretRef(PurpleSecretRef),
    String(String),
}

/// Lifecycle describes actions that the management system should take in response to
/// container lifecycle events. For the PostStart and PreStop lifecycle handlers, management
/// of the container blocks until the action is complete, unless the container process fails,
/// in which case the handler is aborted.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerLifecycle {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleLifecycle(PurpleLifecycle),
    String(String),
}

/// Handler defines a specific action that should be taken
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledPostStart {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurplePostStart(PurplePostStart),
    String(String),
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningExec {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleExec(PurpleExec),
    String(String),
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningHttpGet {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleHttpGet(PurpleHttpGet),
    String(String),
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningHttpHeader {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleHttpHeader(PurpleHttpHeader),
    String(String),
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningTcpSocket {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleTcpSocket(PurpleTcpSocket),
    String(String),
}

/// Handler defines a specific action that should be taken
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledPreStop {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurplePreStop(PurplePreStop),
    String(String),
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaExec {
    Bool(bool),
    Double(f64),
    FluffyExec(FluffyExec),
    Integer(i64),
    String(String),
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaHttpGet {
    Bool(bool),
    Double(f64),
    FluffyHttpGet(FluffyHttpGet),
    Integer(i64),
    String(String),
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaHttpHeader {
    Bool(bool),
    Double(f64),
    FluffyHttpHeader(FluffyHttpHeader),
    Integer(i64),
    String(String),
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaTcpSocket {
    Bool(bool),
    Double(f64),
    FluffyTcpSocket(FluffyTcpSocket),
    Integer(i64),
    String(String),
}

/// Probe describes a health check to be performed against a container to determine whether
/// it is alive or ready to receive traffic.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerLivenessProbe {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleLivenessProbe(PurpleLivenessProbe),
    String(String),
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyExec {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledExec(TentacledExec),
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyHttpGet {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledHttpGet(TentacledHttpGet),
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyHttpHeader {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledHttpHeader(TentacledHttpHeader),
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyTcpSocket {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledTcpSocket(TentacledTcpSocket),
}

/// ContainerPort represents a network port in a single container.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerPort {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurplePort(PurplePort),
    String(String),
}

/// Probe describes a health check to be performed against a container to determine whether
/// it is alive or ready to receive traffic.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerReadinessProbe {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleReadinessProbe(PurpleReadinessProbe),
    String(String),
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousExec {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyExec(StickyExec),
    String(String),
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousHttpGet {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyHttpGet(StickyHttpGet),
    String(String),
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousHttpHeader {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyHttpHeader(StickyHttpHeader),
    String(String),
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousTcpSocket {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyTcpSocket(StickyTcpSocket),
    String(String),
}

/// ResourceRequirements describes the compute resource requirements.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerResources {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleResources(PurpleResources),
    String(String),
}

/// SecurityContext holds security configuration that will be applied to a container. Some
/// fields are present in both SecurityContext and PodSecurityContext.  When both are set,
/// the values in SecurityContext take precedence.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerSecurityContext {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleSecurityContext(PurpleSecurityContext),
    String(String),
}

/// Adds and removes POSIX capabilities from running containers.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledCapabilities {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleCapabilities(PurpleCapabilities),
    String(String),
}

/// SELinuxOptions are the labels to be applied to the container
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickySeLinuxOptions {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleSeLinuxOptions(PurpleSeLinuxOptions),
    String(String),
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerVolumeMount {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleVolumeMount(PurpleVolumeMount),
    String(String),
}

/// LocalObjectReference contains enough information to let you locate the referenced object
/// inside the same namespace.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImagePullSecretElement {
    Bool(bool),
    Double(f64),
    ImagePullSecretClass(ImagePullSecretClass),
    Integer(i64),
    String(String),
}

/// A single application container that you want to run within a pod.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerElement {
    Bool(bool),
    Double(f64),
    InitContainerClass(InitContainerClass),
    Integer(i64),
    String(String),
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerEnv {
    Bool(bool),
    Double(f64),
    FluffyEnv(FluffyEnv),
    Integer(i64),
    String(String),
}

/// EnvVarSource represents a source for the value of an EnvVar.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyValueFrom {
    Bool(bool),
    Double(f64),
    FluffyValueFrom(FluffyValueFrom),
    Integer(i64),
    String(String),
}

/// Selects a key from a ConfigMap.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyConfigMapKeyRef {
    Bool(bool),
    Double(f64),
    FluffyConfigMapKeyRef(FluffyConfigMapKeyRef),
    Integer(i64),
    String(String),
}

/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentFieldRef {
    Bool(bool),
    Double(f64),
    FluffyFieldRef(FluffyFieldRef),
    Integer(i64),
    String(String),
}

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentResourceFieldRef {
    Bool(bool),
    Double(f64),
    FluffyResourceFieldRef(FluffyResourceFieldRef),
    Integer(i64),
    String(String),
}

/// SecretKeySelector selects a key of a Secret.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickySecretKeyRef {
    Bool(bool),
    Double(f64),
    FluffySecretKeyRef(FluffySecretKeyRef),
    Integer(i64),
    String(String),
}

/// EnvFromSource represents the source of a set of ConfigMaps
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerEnvFrom {
    Bool(bool),
    Double(f64),
    FluffyEnvFrom(FluffyEnvFrom),
    Integer(i64),
    String(String),
}

/// ConfigMapEnvSource selects a ConfigMap to populate the environment variables with.
///
/// The contents of the target ConfigMap's Data field will represent the key-value pairs as
/// environment variables.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyConfigMapRef {
    Bool(bool),
    Double(f64),
    FluffyConfigMapRef(FluffyConfigMapRef),
    Integer(i64),
    String(String),
}

/// SecretEnvSource selects a Secret to populate the environment variables with.
///
/// The contents of the target Secret's Data field will represent the key-value pairs as
/// environment variables.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousSecretRef {
    Bool(bool),
    Double(f64),
    FluffySecretRef(FluffySecretRef),
    Integer(i64),
    String(String),
}

/// Lifecycle describes actions that the management system should take in response to
/// container lifecycle events. For the PostStart and PreStop lifecycle handlers, management
/// of the container blocks until the action is complete, unless the container process fails,
/// in which case the handler is aborted.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerLifecycle {
    Bool(bool),
    Double(f64),
    FluffyLifecycle(FluffyLifecycle),
    Integer(i64),
    String(String),
}

/// Handler defines a specific action that should be taken
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyPostStart {
    Bool(bool),
    Double(f64),
    FluffyPostStart(FluffyPostStart),
    Integer(i64),
    String(String),
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BraggadociousExec {
    Bool(bool),
    Double(f64),
    IndigoExec(IndigoExec),
    Integer(i64),
    String(String),
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BraggadociousHttpGet {
    Bool(bool),
    Double(f64),
    IndigoHttpGet(IndigoHttpGet),
    Integer(i64),
    String(String),
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BraggadociousHttpHeader {
    Bool(bool),
    Double(f64),
    IndigoHttpHeader(IndigoHttpHeader),
    Integer(i64),
    String(String),
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BraggadociousTcpSocket {
    Bool(bool),
    Double(f64),
    IndigoTcpSocket(IndigoTcpSocket),
    Integer(i64),
    String(String),
}

/// Handler defines a specific action that should be taken
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyPreStop {
    Bool(bool),
    Double(f64),
    FluffyPreStop(FluffyPreStop),
    Integer(i64),
    String(String),
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Exec1 {
    Bool(bool),
    Double(f64),
    IndecentExec(IndecentExec),
    Integer(i64),
    String(String),
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpGet1 {
    Bool(bool),
    Double(f64),
    IndecentHttpGet(IndecentHttpGet),
    Integer(i64),
    String(String),
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpHeader1 {
    Bool(bool),
    Double(f64),
    IndecentHttpHeader(IndecentHttpHeader),
    Integer(i64),
    String(String),
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TcpSocket1 {
    Bool(bool),
    Double(f64),
    IndecentTcpSocket(IndecentTcpSocket),
    Integer(i64),
    String(String),
}

/// Probe describes a health check to be performed against a container to determine whether
/// it is alive or ready to receive traffic.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerLivenessProbe {
    Bool(bool),
    Double(f64),
    FluffyLivenessProbe(FluffyLivenessProbe),
    Integer(i64),
    String(String),
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Exec2 {
    Bool(bool),
    Double(f64),
    HilariousExec(HilariousExec),
    Integer(i64),
    String(String),
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpGet2 {
    Bool(bool),
    Double(f64),
    HilariousHttpGet(HilariousHttpGet),
    Integer(i64),
    String(String),
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpHeader2 {
    Bool(bool),
    Double(f64),
    HilariousHttpHeader(HilariousHttpHeader),
    Integer(i64),
    String(String),
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TcpSocket2 {
    Bool(bool),
    Double(f64),
    HilariousTcpSocket(HilariousTcpSocket),
    Integer(i64),
    String(String),
}

/// ContainerPort represents a network port in a single container.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerPort {
    Bool(bool),
    Double(f64),
    FluffyPort(FluffyPort),
    Integer(i64),
    String(String),
}

/// Probe describes a health check to be performed against a container to determine whether
/// it is alive or ready to receive traffic.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerReadinessProbe {
    Bool(bool),
    Double(f64),
    FluffyReadinessProbe(FluffyReadinessProbe),
    Integer(i64),
    String(String),
}

/// ExecAction describes a "run in container" action.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Exec3 {
    AmbitiousExec(AmbitiousExec),

    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
}

/// HTTPGetAction describes an action based on HTTP Get requests.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpGet3 {
    AmbitiousHttpGet(AmbitiousHttpGet),

    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
}

/// HTTPHeader describes a custom header to be used in HTTP probes
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpHeader3 {
    AmbitiousHttpHeader(AmbitiousHttpHeader),

    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
}

/// TCPSocketAction describes an action based on opening a socket
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TcpSocket3 {
    AmbitiousTcpSocket(AmbitiousTcpSocket),

    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
}

/// ResourceRequirements describes the compute resource requirements.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerResources {
    Bool(bool),
    Double(f64),
    FluffyResources(FluffyResources),
    Integer(i64),
    String(String),
}

/// SecurityContext holds security configuration that will be applied to a container. Some
/// fields are present in both SecurityContext and PodSecurityContext.  When both are set,
/// the values in SecurityContext take precedence.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerSecurityContext {
    Bool(bool),
    Double(f64),
    FluffySecurityContext(FluffySecurityContext),
    Integer(i64),
    String(String),
}

/// Adds and removes POSIX capabilities from running containers.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyCapabilities {
    Bool(bool),
    Double(f64),
    FluffyCapabilities(FluffyCapabilities),
    Integer(i64),
    String(String),
}

/// SELinuxOptions are the labels to be applied to the container
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndigoSeLinuxOptions {
    Bool(bool),
    Double(f64),
    FluffySeLinuxOptions(FluffySeLinuxOptions),
    Integer(i64),
    String(String),
}

/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitContainerVolumeMount {
    Bool(bool),
    Double(f64),
    FluffyVolumeMount(FluffyVolumeMount),
    Integer(i64),
    String(String),
}

/// PodSecurityContext holds pod-level security attributes and common container settings.
/// Some fields are also present in container.securityContext.  Field values of
/// container.securityContext take precedence over field values of PodSecurityContext.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpecSecurityContext {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledSecurityContext(TentacledSecurityContext),
}

/// SELinuxOptions are the labels to be applied to the container
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndecentSeLinuxOptions {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledSeLinuxOptions(TentacledSeLinuxOptions),
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple
/// <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TolerationElement {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TolerationClass(TolerationClass),
}

/// Volume represents a named volume in a pod that may be accessed by any container in the
/// pod.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeElement {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    VolumeClass(VolumeClass),
}

/// Represents a Persistent Disk resource in AWS.
///
/// An AWS EBS disk must exist before mounting to a container. The disk must also be in the
/// same AWS zone as the kubelet. An AWS EBS disk can only be mounted as read/write once. AWS
/// EBS volumes support ownership management and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AwsElasticBlockStoreUnion {
    AwsElasticBlockStoreClass(AwsElasticBlockStoreClass),
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
}

/// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AzureDiskUnion {
    AzureDiskClass(AzureDiskClass),
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
}

/// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AzureFileUnion {
    AzureFileClass(AzureFileClass),
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
}

/// Represents a Ceph Filesystem mount that lasts the lifetime of a pod Cephfs volumes do not
/// support ownership management or SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CephfsUnion {
    Bool(bool),
    CephfsClass(CephfsClass),
    Double(f64),
    Integer(i64),
    String(String),
}

/// LocalObjectReference contains enough information to let you locate the referenced object
/// inside the same namespace.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CephfsSecretRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledSecretRef(TentacledSecretRef),
}

/// Represents a cinder volume resource in Openstack. A Cinder volume must exist before
/// mounting to a container. The volume must also be in the same region as the kubelet.
/// Cinder volumes support ownership management and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CinderUnion {
    Bool(bool),
    CinderClass(CinderClass),
    Double(f64),
    Integer(i64),
    String(String),
}

/// Adapts a ConfigMap into a volume.
///
/// The contents of the target ConfigMap's Data field will be presented in a volume as files
/// using the keys in the Data field as the file names, unless the items element is populated
/// with specific mappings of keys to paths. ConfigMap volumes support ownership management
/// and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeConfigMap {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleConfigMap(PurpleConfigMap),
    String(String),
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousItem {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleItem(PurpleItem),
    String(String),
}

/// DownwardAPIVolumeSource represents a volume containing downward API info. Downward API
/// volumes support ownership management and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeDownwardApi {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleDownwardApi(PurpleDownwardApi),
    String(String),
}

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousItem {
    Bool(bool),
    Double(f64),
    FluffyItem(FluffyItem),
    Integer(i64),
    String(String),
}

/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousFieldRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledFieldRef(TentacledFieldRef),
}

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HilariousResourceFieldRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledResourceFieldRef(TentacledResourceFieldRef),
}

/// Represents an empty directory for a pod. Empty directory volumes support ownership
/// management and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmptyDirUnion {
    Bool(bool),
    Double(f64),
    EmptyDirClass(EmptyDirClass),
    Integer(i64),
    String(String),
}

/// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as
/// read/write once. Fibre Channel volumes support ownership management and SELinux
/// relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FcUnion {
    Bool(bool),
    Double(f64),
    FcClass(FcClass),
    Integer(i64),
    String(String),
}

/// FlexVolume represents a generic volume resource that is provisioned/attached using an
/// exec based plugin. This is an alpha feature and may change in future.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlexVolumeUnion {
    Bool(bool),
    Double(f64),
    FlexVolumeClass(FlexVolumeClass),
    Integer(i64),
    String(String),
}

/// LocalObjectReference contains enough information to let you locate the referenced object
/// inside the same namespace.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlexVolumeSecretRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickySecretRef(StickySecretRef),
    String(String),
}

/// Represents a Flocker volume mounted by the Flocker agent. One and only one of datasetName
/// and datasetUUID should be set. Flocker volumes do not support ownership management or
/// SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlockerUnion {
    Bool(bool),
    Double(f64),
    FlockerClass(FlockerClass),
    Integer(i64),
    String(String),
}

/// Represents a Persistent Disk resource in Google Compute Engine.
///
/// A GCE PD must exist before mounting to a container. The disk must also be in the same GCE
/// project and zone as the kubelet. A GCE PD can only be mounted as read/write once or
/// read-only many times. GCE PDs support ownership management and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GcePersistentDiskUnion {
    Bool(bool),
    Double(f64),
    GcePersistentDiskClass(GcePersistentDiskClass),
    Integer(i64),
    String(String),
}

/// Represents a volume that is populated with the contents of a git repository. Git repo
/// volumes do not support ownership management. Git repo volumes support SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GitRepoUnion {
    Bool(bool),
    Double(f64),
    GitRepoClass(GitRepoClass),
    Integer(i64),
    String(String),
}

/// Represents a Glusterfs mount that lasts the lifetime of a pod. Glusterfs volumes do not
/// support ownership management or SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GlusterfsUnion {
    Bool(bool),
    Double(f64),
    GlusterfsClass(GlusterfsClass),
    Integer(i64),
    String(String),
}

/// Represents a host path mapped into a pod. Host path volumes do not support ownership
/// management or SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HostPathUnion {
    Bool(bool),
    Double(f64),
    HostPathClass(HostPathClass),
    Integer(i64),
    String(String),
}

/// Represents an ISCSI disk. ISCSI volumes can only be mounted as read/write once. ISCSI
/// volumes support ownership management and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IscsiUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    IscsiClass(IscsiClass),
    String(String),
}

/// Represents an NFS mount that lasts the lifetime of a pod. NFS volumes do not support
/// ownership management or SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NfsUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    NfsClass(NfsClass),
    String(String),
}

/// PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This
/// volume finds the bound PV and mounts that volume for the pod. A
/// PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of
/// volume that is owned by someone else (the system).
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PersistentVolumeClaimUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PersistentVolumeClaimClass(PersistentVolumeClaimClass),
    String(String),
}

/// Represents a Photon Controller persistent disk resource.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PhotonPersistentDiskUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PhotonPersistentDiskClass(PhotonPersistentDiskClass),
    String(String),
}

/// PortworxVolumeSource represents a Portworx volume resource.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortworxVolumeUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PortworxVolumeClass(PortworxVolumeClass),
    String(String),
}

/// Represents a projected volume source
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectedUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    ProjectedClass(ProjectedClass),
    String(String),
}

/// Projection that may be projected along with other supported volume types
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceElement {
    Bool(bool),
    Double(f64),
    Integer(i64),
    SourceClass(SourceClass),
    String(String),
}

/// Adapts a ConfigMap into a projected volume.
///
/// The contents of the target ConfigMap's Data field will be presented in a projected volume
/// as files using the keys in the Data field as the file names, unless the items element is
/// populated with specific mappings of keys to paths. Note that this is identical to a
/// configmap volume source without the default mode.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceConfigMap {
    Bool(bool),
    Double(f64),
    FluffyConfigMap(FluffyConfigMap),
    Integer(i64),
    String(String),
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CunningItem {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    TentacledItem(TentacledItem),
}

/// Represents downward API info for projecting into a projected volume. Note that this is
/// identical to a downwardAPI volume source without the default mode.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceDownwardApi {
    Bool(bool),
    Double(f64),
    FluffyDownwardApi(FluffyDownwardApi),
    Integer(i64),
    String(String),
}

/// DownwardAPIVolumeFile represents information to create the file containing the pod field
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MagentaItem {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyItem(StickyItem),
    String(String),
}

/// ObjectFieldSelector selects an APIVersioned field of an object.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousFieldRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyFieldRef(StickyFieldRef),
    String(String),
}

/// ResourceFieldSelector represents container resources (cpu, memory) and their output format
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AmbitiousResourceFieldRef {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StickyResourceFieldRef(StickyResourceFieldRef),
    String(String),
}

/// Adapts a secret into a projected volume.
///
/// The contents of the target Secret's Data field will be presented in a projected volume as
/// files using the keys in the Data field as the file names. Note that this is identical to
/// a secret volume source without the default mode.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceSecret {
    Bool(bool),
    Double(f64),
    Integer(i64),
    PurpleSecret(PurpleSecret),
    String(String),
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FriskyItem {
    Bool(bool),
    Double(f64),
    IndigoItem(IndigoItem),
    Integer(i64),
    String(String),
}

/// Represents a Quobyte mount that lasts the lifetime of a pod. Quobyte volumes do not
/// support ownership management or SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuobyteUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    QuobyteClass(QuobyteClass),
    String(String),
}

/// Represents a Rados Block Device mount that lasts the lifetime of a pod. RBD volumes
/// support ownership management and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RbdUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    RbdClass(RbdClass),
    String(String),
}

/// LocalObjectReference contains enough information to let you locate the referenced object
/// inside the same namespace.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RbdSecretRef {
    Bool(bool),
    Double(f64),
    IndigoSecretRef(IndigoSecretRef),
    Integer(i64),
    String(String),
}

/// ScaleIOVolumeSource represents a persistent ScaleIO volume
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScaleIoUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    ScaleIoClass(ScaleIoClass),
    String(String),
}

/// LocalObjectReference contains enough information to let you locate the referenced object
/// inside the same namespace.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScaleIoSecretRef {
    Bool(bool),
    Double(f64),
    IndecentSecretRef(IndecentSecretRef),
    Integer(i64),
    String(String),
}

/// Adapts a Secret into a volume.
///
/// The contents of the target Secret's Data field will be presented in a volume as files
/// using the keys in the Data field as the file names. Secret volumes support ownership
/// management and SELinux relabeling.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeSecret {
    Bool(bool),
    Double(f64),
    FluffySecret(FluffySecret),
    Integer(i64),
    String(String),
}

/// Maps a string key to a path within a volume.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MischievousItem {
    Bool(bool),
    Double(f64),
    IndecentItem(IndecentItem),
    Integer(i64),
    String(String),
}

/// Represents a vSphere volume resource.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum VsphereVolumeUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    String(String),
    VsphereVolumeClass(VsphereVolumeClass),
}

/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatusUnion {
    Bool(bool),
    Double(f64),
    Integer(i64),
    StatusClass(StatusClass),
    String(String),
}

/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionElement {
    Bool(bool),
    ConditionClass(ConditionClass),
    Double(f64),
    Integer(i64),
    String(String),
}

fn serialize_config(src: &str, expected: &str) {
    let cfg: Deployment = serde_yaml::from_str(src).unwrap();
    let serialized = konfig::to_string(&cfg).unwrap();

    assert_eq!(serialized, expected);
}

#[test]
fn cfg1() {
    serialize_config(
        include_str!("data/k8s_cfg1.src.yaml"),
        include_str!("data/expected/k8s_cfg1.konfig.md"),
    );
}

#[test]
fn cfg2() {
    serialize_config(
        include_str!("data/k8s_cfg2.src.yaml"),
        include_str!("data/expected/k8s_cfg2.konfig.md"),
    );
}
