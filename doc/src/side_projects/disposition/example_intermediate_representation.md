# Example Intermediate Representation

This is the computed data structure from combining the layered values from the input data.

````yaml
---
# Example Intermediate Representation
#
# This is the computed data from combining the layered values from
# the example input.

# Everything that needs to be represented as a `node` on the diagram, including:
#
# * `things`
# * `tags`
# * `processes`
# * `process_steps`
nodes: &nodes
  #
  # things
  t_aws: "☁️ Amazon Web Services"
  t_aws_iam: "🖊️ Identity and Access Management"
  t_aws_iam_ecs_policy: "🗒️ ECS IAM Policy"
  t_aws_ecr: "🗄️ Elastic Container Registry"
  t_aws_ecr_repo: "💽 web_app repo"
  t_aws_ecr_repo_image_1: "💿 Image 1"
  t_aws_ecr_repo_image_2: "💿 Image 2"
  t_aws_ecs: "💻 Elastic Container Service"
  t_aws_ecs_cluster_app: "🎛️ web_app cluster"
  t_aws_ecs_cluster_app_task: "🖨️ web_app task version 1"
  t_github: "🐙 GitHub"
  t_github_user_repo: "azriel91/web_app"
  t_localhost: "🧑‍💻 Localhost"
  t_localhost_repo: "📂 ~/work/web_app"
  t_localhost_repo_src: "📝 src"
  t_localhost_repo_target: "📂 target"
  t_localhost_repo_target_file_zip: "📝 file.zip"
  t_localhost_repo_target_dist_dir: "📁 dist"

  # tags
  tag_app_development: "Application Development"
  tag_deployment: "Deployment"

  # processes
  proc_app_dev: "App Development"
  proc_app_release: "App Release"
  proc_i12e_region_tier_app_deploy: "Prod App Deploy"

  # steps
  proc_app_dev_step_repository_clone: "Clone repository"
  proc_app_dev_step_project_build: "Build project"
  proc_app_release_step_crate_version_update: "Update crate versions"
  proc_app_release_step_pull_request_open: "Open PR"
  proc_app_release_step_tag_and_push: "Tag and push"
  proc_app_release_step_gh_actions_build: "Github Actions build"
  proc_app_release_step_gh_actions_publish: "Github Actions publish"
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: "Update ECS service"

# Text to copy to clipboard when a node's copy button is clicked.
node_copy_text:
  <<: *nodes
  t_aws: "Amazon Web Services"
  t_aws_iam: "Identity and Access Management"
  t_aws_iam_ecs_policy: "ECS IAM Policy"
  t_aws_ecr: "Elastic Container Registry"
  t_aws_ecr_repo: "web_app repo"
  t_aws_ecr_repo_image_1: "Image 1"
  t_aws_ecr_repo_image_2: "Image 2"
  t_aws_ecs: "Elastic Container Service"
  t_aws_ecs_cluster_app: "web_app cluster"
  t_aws_ecs_cluster_app_task: "web_app task version 1"
  t_github: "GitHub"
  t_github_user_repo: "azriel91/web_app"
  t_localhost: "Localhost"
  t_localhost_repo: "~/work/web_app"
  t_localhost_repo_src: "~/work/web_app/src"
  t_localhost_repo_target: "~/work/web_app/target"
  t_localhost_repo_target_file_zip: "~/work/web_app/target/file.zip"
  t_localhost_repo_target_dist_dir: "~/work/web_app/target/dist"

# Hierarchy of all nodes.
node_hierarchy:
  # Tags before everything else
  tag_app_development: {}
  tag_deployment: {}

  # Processes before things/edges
  proc_app_dev:
    proc_app_dev_step_repository_clone: {}
    proc_app_dev_step_project_build: {}
  proc_app_release:
    proc_app_release_step_crate_version_update: {}
    proc_app_release_step_pull_request_open: {}
    proc_app_release_step_tag_and_push: {}
    proc_app_release_step_gh_actions_build: {}
    proc_app_release_step_gh_actions_publish: {}
  proc_i12e_region_tier_app_deploy:
    proc_i12e_region_tier_app_deploy_step_ecs_service_update: {}

  # Things
  t_aws:
    t_aws_iam:
      t_aws_iam_ecs_policy: {}
    t_aws_ecr:
      t_aws_ecr_repo:
        t_aws_ecr_repo_image_1: {}
        t_aws_ecr_repo_image_2: {}
    t_aws_ecs:
      t_aws_ecs_cluster_app:
        t_aws_ecs_cluster_app_task: {}
  t_github:
    t_github_user_repo: {}
  t_localhost:
    t_localhost_repo:
      t_localhost_repo_src: {}
      t_localhost_repo_target:
        t_localhost_repo_target_file_zip: {}
        t_localhost_repo_target_dist_dir: {}

# Edge groups with explicit from/to edges.
edge_groups:
  edge_t_localhost__t_github_user_repo:
    - from: t_github_user_repo
      to: t_localhost
    - from: t_localhost
      to: t_github_user_repo
  edge_t_localhost__t_github_user_repo__push:
    - from: t_localhost
      to: t_github_user_repo
  edge_t_localhost__t_localhost__within:
    - from: t_localhost
      to: t_localhost
  edge_t_github_user_repo__t_github_user_repo__within:
    - from: t_github_user_repo
      to: t_github_user_repo
  edge_t_github_user_repo__t_aws_ecr_repo__push:
    - from: t_github_user_repo
      to: t_aws_ecr_repo
  edge_t_aws_ecr_repo__t_aws_ecs_service__push:
    - from: t_aws_ecr_repo
      to: t_aws_ecs_service

# Descriptions for entities.
entity_descs:
  # nodes
  # process_steps
  proc_app_release_step_crate_version_update: |-
    ```bash
    sd -s 'version = "0.3.0"' 'version = "0.3.0"' $(fd -tf -F toml) README.md src/lib.rs
    ```
  proc_app_release_step_pull_request_open: |-
    Create a pull request as usual.
  proc_app_release_step_gh_actions_build: |-
    Github Actions will build the image.
  proc_app_release_step_tag_and_push: |-
    When the PR is merged, tag the commit and push the tag to GitHub.

    ```bash
    git tag 0.3.0
    git push origin 0.3.0
    ```

    The build will push the new version to ECR automatically.
  proc_app_release_step_gh_actions_publish: |-
    Github Actions will publish the image to AWS ECR.
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: |-
    Deploy or update the existing ECS service with the new image.

  # edge groups
  edge_t_localhost__t_github_user_repo__pull: "Fetch from GitHub"
  edge_t_localhost__t_github_user_repo__push: "Push to GitHub"

  # edges
  edge_t_localhost__t_github_user_repo__pull__0: "`git pull`"
  edge_t_localhost__t_github_user_repo__push__0: "`git push`"

# Entity types for styling.
entity_types:
  # things
  t_aws: [type_thing_default, type_organisation]
  t_aws_iam: [type_thing_default, type_service]
  t_aws_iam_ecs_policy: [type_thing_default]
  t_aws_ecr: [type_thing_default, type_service]
  t_aws_ecr_repo: [type_thing_default]
  t_aws_ecr_repo_image_1: [type_thing_default, type_docker_image]
  t_aws_ecr_repo_image_2: [type_thing_default, type_docker_image]
  t_aws_ecs: [type_thing_default, type_service]
  t_aws_ecs_cluster_app: [type_thing_default]
  t_aws_ecs_cluster_app_task: [type_thing_default]
  t_github: [type_thing_default, type_organisation]
  t_github_user_repo: [type_thing_default]
  t_localhost: [type_thing_default]
  t_localhost_repo: [type_thing_default]
  t_localhost_repo_src: [type_thing_default]
  t_localhost_repo_target: [type_thing_default]
  t_localhost_repo_target_file_zip: [type_thing_default]
  t_localhost_repo_target_dist_dir: [type_thing_default]

  # tags
  tag_app_development: [tag_type_default]
  tag_deployment: [tag_type_default]

  # processes
  proc_app_dev: [type_process_default]
  proc_app_release: [type_process_default]
  proc_i12e_region_tier_app_deploy: [type_process_default]

  # process steps
  proc_app_dev_step_repository_clone: [type_process_step_default]
  proc_app_dev_step_project_build: [type_process_step_default]
  proc_app_release_step_crate_version_update: [type_process_step_default]
  proc_app_release_step_pull_request_open: [type_process_step_default]
  proc_app_release_step_tag_and_push: [type_process_step_default]
  proc_app_release_step_gh_actions_build: [type_process_step_default]
  proc_app_release_step_gh_actions_publish: [type_process_step_default]
  proc_i12e_region_tier_app_deploy_step_ecs_service_update: [type_process_step_default]

  # edges
  #
  # These IDs are the `edge_group` IDs suffixed with "__" and the edge's index within the group.
  edge_t_localhost__t_github_user_repo__pull__0:
    [type_edge_dependency_cyclic_default, type_edge_interaction_cyclic_default]
  edge_t_localhost__t_github_user_repo__pull__1:
    [type_edge_dependency_cyclic_default, type_edge_interaction_cyclic_default]
  edge_t_localhost__t_github_user_repo__push__0:
    [type_edge_dependency_sequence_request_default, type_edge_interaction_sequence_request_default]
  edge_t_localhost__t_localhost__within__0:
    [type_edge_dependency_cyclic_default, type_edge_interaction_cyclic_default]
  edge_t_github_user_repo__t_github_user_repo__within:
    [type_edge_dependency_cyclic_default, type_edge_interaction_cyclic_default]
  edge_t_github_user_repo__t_aws_ecr_repo__push__0:
    [type_edge_dependency_sequence_request_default, type_edge_interaction_sequence_request_default]
  edge_t_aws_ecr_repo__t_aws_ecs_service__push__0:
    [type_edge_dependency_sequence_request_default, type_edge_interaction_sequence_request_default]

# Computed Tailwind CSS classes.
tailwind_classes:
  tag_app_development: >-
    stroke-1
    visible
    hover:fill-emerald-400
    fill-emerald-500
    focus:fill-emerald-600
    active:fill-emerald-700
    peer/tag_app_development

  tag_deployment: >-
    stroke-1
    visible
    hover:fill-emerald-400
    fill-emerald-500
    peer/tag_deployment

  proc_app_dev: >-
    stroke-1
    visible
    hover:fill-blue-200
    fill-blue-300
    group/proc_app_dev

  proc_app_release: >-
    stroke-1
    visible
    hover:fill-blue-200
    fill-blue-300
    group/proc_app_release

  proc_i12e_region_tier_app_deploy: >-
    stroke-1
    visible
    hover:fill-blue-200
    fill-blue-300
    group/proc_i12e_region_tier_app_deploy

  proc_app_dev_step_repository_clone: >-
    stroke-1
    invisible
    hover:fill-sky-200
    fill-sky-300
    peer/proc_app_dev_step_repository_clone
    group-focus-within/proc_app_dev:visible

  proc_app_dev_step_project_build: >-
    stroke-1
    invisible
    hover:fill-sky-200
    fill-sky-300
    peer/proc_app_dev_step_project_build
    group-focus-within/proc_app_dev:visible

  t_aws: >-
    [stroke-dasharray:2]
    stroke-1
    visible
    hover:fill-yellow-50
    fill-yellow-100
    [&>text]:fill-neutral-800

  t_github: >-
    [stroke-dasharray:3]
    stroke-1
    visible
    hover:fill-neutral-50
    fill-neutral-100
    [&>text]:fill-neutral-800

  t_localhost: >-
    stroke-1
    visible
    hover:fill-slate-200
    fill-slate-300
    [&>text]:fill-neutral-900

# Layout configuration for nodes.
node_layout:
  _root:
    flex:
      direction: "column_reverse"
      wrap: true
      gap: "4"

  _things_and_processes_container:
    flex:
      direction: "row_reverse"
      wrap: true
      gap: "4"

  _processes_container:
    flex:
      direction: "row"
      wrap: true
      gap: "4"

  proc_app_dev:
    flex:
      direction: "column"
      wrap: false
      gap: "2"

  proc_app_release:
    flex:
      direction: "column"
      wrap: false
      gap: "2"

  proc_i12e_region_tier_app_deploy:
    flex:
      direction: "column"
      wrap: false
      gap: "2"

  proc_app_dev_step_repository_clone: none
  proc_app_dev_step_project_build: none

  _tags_container:
    flex:
      direction: "row"
      wrap: true
      gap: "2"

  tag_app_development: none
  tag_deployment: none

  _things_container:
    flex:
      direction: "row"
      wrap: true
      gap: "4"

  t_aws:
    flex:
      direction: "column"
      wrap: false
      gap: "2"

  t_github:
    flex:
      direction: "row"
      wrap: false
      gap: "2"

  t_github_user_repo: none

  t_localhost:
    flex:
      direction: "row"
      wrap: false
      gap: "2"

# Additional CSS for animations.
css: >-
  @keyframes stroke-dashoffset-move {
    0%   { stroke-dasharray: 3; stroke-dashoffset: 30; }
    100% { stroke-dasharray: 3; stroke-dashoffset: 0; }
  }
  @keyframes stroke-dashoffset-move-request {
    0%   { stroke-dashoffset: 0; }
    100% { stroke-dashoffset: 228; }
  }
````
