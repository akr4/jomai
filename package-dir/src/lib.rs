pub use bower::is_bower_package_dir;
pub use bundler::is_bundler_package_dir;
pub use chef::is_chef_cookbook_dir;
pub use cocoapods::is_cocoapods_pods_dir;
pub use composer::is_composer_package_dir;
pub use npm::is_npm_package_dir;
pub use python::is_python_package_dir;

mod bower;
mod bundler;
mod chef;
mod cocoapods;
mod composer;
mod npm;
mod python;
