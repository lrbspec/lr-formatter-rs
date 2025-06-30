use std::collections::HashSet;

/// Error message that expects a feature's data to be initialized if the feature flag was enabled
pub(super) const UNREACHABLE_MESSAGE: &'static str =
    "BUG: Feature data should have been initialized when including feature flag";

/// Implement this trait to include methods that access optional fields enabled by feature flags
pub(super) trait FeatureFieldAccess<Feature, Error> {
    /// Requires an optional feature to exist by checking feature flags and returning an immutable
    /// reference to the unwrapped feature
    fn require_feature<'a, T>(
        &self,
        field: &'a Option<T>,
        feature: Feature,
    ) -> Result<&'a T, Error>;

    /// Requires an optional feature to exist by checking feature flags and returning a mutable
    /// reference to the unwrapped feature
    fn require_feature_mut<'a, T>(
        current_features: &HashSet<Feature>,
        field: &'a mut Option<T>,
        feature: Feature,
    ) -> Result<&'a mut T, Error>;

    /// Compares a feature's data and flag by checking that either both are present or absent
    fn check_feature<T>(
        &self,
        feature: Feature,
        field: &Option<T>,
        attr_name: &'static str,
    ) -> Result<(), Error>;
}
