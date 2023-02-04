use super::identity::Identity;

pub trait Props {
    fn prop(&self) -> &Self;
}

pub trait Entity<I: Identity, P: Props> {
    fn new(id: I, props: P) -> Self;
    fn props(&self) -> &P;
}

pub trait IAnaProps: Props {
    fn name(&self) -> &str;
}

#[derive(Clone, Debug)]
pub struct AnaProps {
    _name: String,
}

impl Props for AnaProps {
    fn prop(&self) -> &Self {
        self
    }
}

impl IAnaProps for AnaProps {
    fn name(&self) -> &str {
        &self._name
    }
}

#[derive(Debug, Clone)]
pub struct Ana<I, P> {
    _id: I,
    _props: P,
}

impl<I, P: IAnaProps> Ana<I, P> {
    pub fn name(&self) -> &str {
        self._props.name()
    }
}

impl<I: Identity, P: Props> Entity<I, P> for Ana<I, P> {
    fn new(id: I, props: P) -> Self {
        Self {
            _id: id,
            _props: props,
        }
    }

    fn props(&self) -> &P {
        &self._props
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::identity::UniqueIdInt;

    #[test]
    fn should_create_ana() {
        let id = UniqueIdInt::new(1);
        let props = AnaProps {
            _name: "Aninha".to_string(),
        };
        let ana: Ana<UniqueIdInt, AnaProps> = Ana::new(id, props);

        assert_eq!(ana.name(), "Aninha");
    }
}
