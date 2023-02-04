use super::identity::Identity;
use crate::identity::UniqueIdInt;

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
pub struct Ana<I: Identity, P> {
    _id: I,
    _props: P,
}

impl<I: Identity, P: IAnaProps> Ana<I, P> {
    pub fn name(&self) -> &str {
        self._props.name()
    }

    pub fn id(&self) -> &I {
        &self._id
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

type AnaImpl = Ana<UniqueIdInt, AnaProps>;

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn should_create_ana() {
        let id = UniqueIdInt::new(1);
        let props = AnaProps {
            _name: "Aninha".to_string(),
        };
        let ana: AnaImpl = AnaImpl::new(id, props);

        assert_eq!(ana.name(), "Aninha");
        assert_eq!(ana.id().to_string(), "1");
    }
}
